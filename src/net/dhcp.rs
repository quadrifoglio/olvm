/*
 * DHCP module - Handle DHCP requests from VMs
 */

use std::error::Error as StdError;
use std::net::{UdpSocket, SocketAddr};

use dhcp::codes;
use dhcp::common::{Frame, Option};

use common::{Result, Error};

pub fn listen() -> Result<()> {
    // Bind the socket
    let socket = match UdpSocket::bind("0.0.0.0:67") {
        Ok(socket) => socket,
        Err(e) => return Err(Error::new(e.description()))
    };

    // Forever
    loop {
        // 1024 bytes buffer
        let mut buf = [0; 1024];

        // On each datagram
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                // Handle the request
                let frame = match Frame::parse(&buf[..len]) {
                    Ok(frame) => handle(&socket, src, frame),
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

fn handle(socket: &UdpSocket, src: SocketAddr, req: Frame) {
    let mut resp = Frame::new(codes::BOOTP_RESPONSE, req.xid);

    // Set the subnet mask
    let mut mask = Option::new(codes::OPTION_SUBNET_MASK);
    mask.set_data(vec![255, 255, 255, 0]);
    resp.add_option(mask);

    // Set the router
    let mut router = Option::new(codes::OPTION_ROUTER);
    router.set_data(vec![192, 168, 1, 254]);
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

    match resp.to_bytes() {
        Ok(buf) => {
            match socket.send_to(buf.as_slice(), src) {
                Ok(_) => {},
                Err(e) => println!("Failed to send DHCP response: {}", e)
            };
        },
        Err(e) => {
            println!("Failed to construct DHCP response: {}", e)
        }
    };
}
