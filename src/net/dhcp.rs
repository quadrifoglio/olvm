/*
 * DHCP module - Handle DHCP requests from VMs
 */

use std::error::Error as StdError;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};

use dhcp::codes;
use dhcp::common::{Frame, Option};

use common::{Result, Error};

pub fn listen() -> Result<()> {
    // Bind the socket
    let socket = match UdpSocket::bind("0.0.0.0:67") {
        Ok(socket) => socket,
        Err(e) => return Err(Error::new(e.description()))
    };

    try!(socket.set_broadcast(true));

    // Forever
    loop {
        // 1024 bytes buffer
        let mut buf = [0; 1024];

        // On each datagram
        match socket.recv_from(&mut buf) {
            Ok((len, _)) => {
                // Handle the request
                let frame = match Frame::parse(&buf[..len]) {
                    Ok(frame) => handle(&socket, frame),
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

fn handle(socket: &UdpSocket, req: Frame) {
    // Constructs a new DHCP response
    let mut resp = Frame::response(req.xid, req.chaddr.clone(), vec![192, 168, 1, 1], vec![192, 168, 1, 253]);

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
    let mut router = Option::new(codes::OPTION_ROUTER);
    router.set_data_ip(192, 168, 1, 254);
    resp.add_option(router);

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
