/*
 * UDP Interface
 */

use std::error::Error as StdError;
use std::net::{UdpSocket, SocketAddr};
use std::process::{self};

use common::{Result, Error};
use handler::{self};

use mongodb::db::Database;

fn send(socket: &UdpSocket, dst: SocketAddr, mut buf: String) -> Result<()> {
    // Add a newline to improve the client's output
    buf.push('\n');

    match socket.send_to(buf.as_bytes(), &dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(format!("Failed to send UDP packet: {}", e)))
    }
}

fn command(db: &Database, socket: &UdpSocket, src: SocketAddr, buf: Vec<u8>) -> Result<()> {
    let s = try!(String::from_utf8(buf).ok().ok_or(Error::new("Failed to read string from UDP packet")));
    let (command, obj) = super::parse_command(s);

    try!(match handler::handle(db, command.as_str(), obj.as_str()) {
        Ok(result) => send(socket, src, result),
        Err(e) => send(socket, src, e.description().to_string())
    });

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
