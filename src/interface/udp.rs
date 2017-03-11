/*
 * UDP interface - Read commands from a listening UDP socket
 */

use std::error::Error as StdError;
use std::net::{UdpSocket, SocketAddr};
use std::process;

use common::{Context, Result, Error};
use handler;

fn send(socket: &UdpSocket, dst: SocketAddr, mut buf: String) -> Result<()> {
    // Add a newline to improve the client's output
    buf.push('\n');

    // Send response to the client
    match socket.send_to(buf.as_bytes(), &dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(format!("Failed to send UDP packet: {}", e)))
    }
}

fn command(ctx: &Context, socket: &UdpSocket, src: SocketAddr, buf: Vec<u8>) -> Result<()> {
    // Parse and handle the command
    let s = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => return Err(Error::new(format!("Read string from UDP packet: {}", e)))
    };

    let (command, obj) = super::parse_command(s);

    // Ignore empty commands
    if command.len() == 0 {
        return Ok(());
    }

    let client = format!("UDP {}", src);

    try!(match handler::handle(ctx, client.as_str(), command.as_str(), obj.as_str()) {
        Ok(result) => send(socket, src, result),
        Err(e) => send(socket, src, e.description().to_string())
    });

    Ok(())
}

pub fn run(ctx: &Context) {
    // 1024 bytes buffer
    let mut buf = vec![0; 1024];

    let addr = match ctx.conf.udp {
        Some(ref udp) => udp.addr.clone(),
        None => {
            println!("Please specify a UDP listen address in configuration");
            process::exit(1);
        }
    };

    // Bind a listen socket
    let socket = match UdpSocket::bind(addr.as_str()) {
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
                match command(ctx, &socket, src, buf[..len].to_vec()) {
                    Ok(_) => {},
                    Err(e) => println!("Failed to execute command: {}", e)
                }
            },
            Err(e) => println!("{}", e)
        };
    }
}
