/*
 * DHCP module - Handle DHCP requests from VMs
 */

use std::error::Error as StdError;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::sync::Arc;

use dhcp::codes;
use dhcp::common::{Frame, Option};

use database;
use common::{Context, Result, Error};

pub fn listen(ctx: Arc<Context>) -> Result<()> {
    // Bind the socket
    let socket = match UdpSocket::bind("0.0.0.0:67") {
        Ok(socket) => socket,
        Err(e) => return Err(Error::new(e.description()))
    };

    try!(socket.set_broadcast(true));

    println!("DHCP server started...");

    // Forever
    loop {
        // 1024 bytes buffer
        let mut buf = [0; 1024];

        // On each datagram
        match socket.recv_from(&mut buf) {
            Ok((len, _)) => {
                // Handle the request
                match Frame::parse(&buf[..len]) {
                    Ok(frame) => handle(ctx.clone(), &socket, frame),
                    Err(e) => {
                        println!("Failed to parse DHCP frame: {}", e);
                        continue;
                    }
                };
            },
            Err(e) => return Err(Error::new(e.description()))
        }
    }

}

fn handle(ctx: Arc<Context>, socket: &UdpSocket, req: Frame) {
    // Get the VM and its interface from database
    let (vm, index) = match database::vm::get_mac(ctx.as_ref(), req.client_mac_string().as_str()) {
        Ok((vm, index)) => (vm, index),
        Err(_) => return // Ignore the request if the MAC address is not found
    };

    let iface = vm.interfaces.get(index).unwrap(); // Unwrapping is ok, checked in database::vm::get_mac

    let net = match database::network::get(ctx.as_ref(), iface.network.as_str()) {
        Ok(net) => net,
        Err(e) => {
            println!("Failed to find network: {}", e);
            return;
        }
    };

    // Get the VM's IP address
    let ip = match Ipv4Addr::from_str(iface.ip.as_str()) {
        Ok(ip) => ip,
        Err(_) => {
            println!("Failed to parse IP: {}", iface.ip);
            return;
        }
    };

    // Constructs a new DHCP response
    let mut resp = Frame::response(req.xid, req.chaddr.clone(), ip.octets().to_vec(), vec![192, 168, 1, 253]);

    let req_type = match req.option(codes::OPTION_DHCP_MSG_TYPE) {
        Some(opt) => {
            let ref data = opt.data;
            data[0]
        },
        None => {
            println!("Invalid DHCP request: missing OPTION_DHCP_MSG_TYPE");
            return;
        }
    };

    let t = match req_type {
        codes::DHCP_DISCOVER => {
            // If its a DHCP Discover, reply with DHCP Offer
            let mut t = Option::new(codes::OPTION_DHCP_MSG_TYPE);
            t.set_data_u8(codes::DHCP_OFFER);

            t
        },
        codes::DHCP_REQUEST => {
            // If its a DHCP Request, reply with DHCP ACK
            let mut t = Option::new(codes::OPTION_DHCP_MSG_TYPE);
            t.set_data_u8(codes::DHCP_ACK);

            t
        },
        _ => {
            println!("Invalid DHCP request: invalid OPTION_DHCP_MSG_TYPE: {}", req_type);
            return;
        }
    };

    resp.add_option(t);

    // Set the subnet mask
    let mut mask = Option::new(codes::OPTION_SUBNET_MASK);
    mask.set_data_ip(255, 255, 255, 0);
    resp.add_option(mask);

    // Set the router
    match net.router.len() {
        0 => {},
        _ => {
            match Ipv4Addr::from_str(net.router.as_str()) {
                Ok(ip) => {
                    let mut router = Option::new(codes::OPTION_ROUTER);
                    router.set_data(ip.octets().to_vec());
                    resp.add_option(router);
                },
                Err(_) => {
                    println!("Failed to parse IP: {}", iface.ip);
                    return;
                }
            };
        }
    };

    // Set the DNS
    match net.dns.len() {
        0 => {},
        _ => {
            match Ipv4Addr::from_str(net.dns[0].as_str()) {
                Ok(ip) => {
                    let mut router = Option::new(codes::OPTION_DOMAIN_SERVER);
                    router.set_data(ip.octets().to_vec());
                    resp.add_option(router);
                },
                Err(_) => {
                    println!("Failed to parse IP: {}", iface.ip);
                    return;
                }
            };
        }
    }

    // Set the lease time
    let mut lease = Option::new(codes::OPTION_ADDRESS_LEASE_TIME);
    match lease.set_data_u32(86400) {
        Ok(_) => resp.add_option(lease),
        Err(e) => {
            println!("Failed to construct DHCP response: {}", e);
            return;
        }
    };

    resp.add_option(Option::new(codes::OPTION_END));

    match resp.to_bytes() {
        Ok(buf) => {
            // Broadcast the response
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 68);
            let data = buf.as_slice();

            match socket.send_to(data, addr) {
                Ok(_) => {},
                Err(e) => println!("Failed to send DHCP response: {}", e)
            };
        },
        Err(e) => {
            println!("Failed to construct DHCP response: {}", e)
        }
    };
}
