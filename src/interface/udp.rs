/*
 * UDP interface - Read commands from a listening UDP socket
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

    // Send response to the client
    match socket.send_to(buf.as_bytes(), &dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(format!("Failed to send UDP packet: {}", e)))
    }
}

fn command(db: &Database, socket: &UdpSocket, src: SocketAddr, buf: Vec<u8>) -> Result<()> {
    // Parse and handle the command
    let s = try!(String::from_utf8(buf).ok().ok_or(Error::new("Failed to read string from UDP packet")));
    let (command, obj) = super::parse_command(s);

    // Ignore empty commands
    if command.len() == 0 {
        return Ok(());
    }

    try!(match handler::handle(db, command.as_str(), obj.as_str()) {
        Ok(result) => send(socket, src, result),
        Err(e) => send(socket, src, e.description().to_string())
    });

    Ok(())
}

pub fn run(addr: &str, db: &Database) {
    // 1024 bytes buffer
    let mut buf = vec![0; 1024];

    // Bind a listen socket
    let socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind socket: {}", e);
            process::exit(1);
        }
    };

    println!("Waiting for commands on UDP {}...", addr);

    // Read all the UDP packets and parse their content
    loop {
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
