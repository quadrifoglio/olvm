/*
 * UDP Interface
 */

use std::net::{UdpSocket, SocketAddr};
use std::process::{self};

use common::{Result, Error};
use handler::{self};

use mongodb::db::Database;

fn command(db: &Database, socket: &UdpSocket, src: SocketAddr, buf: Vec<u8>) -> Result<()> {
    let s = try!(String::from_utf8(buf).ok().ok_or(Error::new("Failed to read string from UDP packet")));
    let (command, obj) = super::parse_command(s);
    let result = try!(handler::handle(db, command.as_str(), obj.as_str()));

    try!(socket.send_to(result.as_bytes(), &src).ok().ok_or(Error::new("Failed to send UDP packet")));

    Ok(())
}

pub fn run(addr: &str, db: &Database) {
    let socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind socket: {}", e);
            process::exit(1);
        }
    };

    loop {
        let mut buf = vec![0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                match command(db, &socket, src, buf[..len].to_vec()) {
                    Ok(_) => {},
                    Err(e) => println!("Failed to execute command: {}", e)
                }
            },
            Err(e) => println!("{}", e)
        };
    }
}
