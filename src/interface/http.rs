/*
 * UDP interface - Read commands from a listening UDP socket
 */

use std::error::Error as StdError;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Write};
use std::sync::Arc;
use std::process;
use std::thread;

use mhttp::Request;

use common::{Context, Result, Error};
use handler;

/*
 * Return an HTTP error to the client
 */
fn response_error(socket: &mut TcpStream, status: &str, body: &str) -> Result<()> {
    let resp = format!("HTTP/1.1 {}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", status, body.len(), body);

    match socket.write(resp.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(e.description()))
    }
}

/*
 * Return a 200 OK response to the client
 */
fn response_ok(socket: &mut TcpStream, body: &str) -> Result<()> {
    let resp = format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);

    match socket.write(resp.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(e.description()))
    }
}

/*
 * Main client loop
 */
fn client(ctx: &Context, mut socket: TcpStream) -> Result<()> {
    loop {
        let req: Request;
        {
            let mut r = BufReader::new(&socket);
            req = match Request::parse(&mut r) {
                Ok(req) => req,
                Err(_) => return Ok(())
            };
        }

        if req.url.len() < 2 {
            return response_error(&mut socket, "400 Bad Request", "{\"error\": \"Please specify the command in the URL\"}")
        }

        let client = format!("HTTP {}", try!(socket.peer_addr()));

        let body = match String::from_utf8(req.body) {
            Ok(body) => body.trim().to_string(),
            Err(_) => return Ok(())
        };

        try!(match handler::handle(ctx, client.as_str(), &req.url[1..], body.as_str()) {
            Ok(result) => response_ok(&mut socket, result.as_str()),
            Err(e) => response_error(&mut socket, "500 Internal Server Error", e.description_json().as_str())
        });
    }
}

pub fn run(ctx: Arc<Context>) {
    // Retreive the listen address
    let addr = match ctx.conf.http {
        Some(ref http) => http.addr.clone(),
        None => {
            println!("Please specify an HTTP listen address in configuration");
            process::exit(1);
        }
    };

    // Bind a listen socket
    let listener = match TcpListener::bind(addr.as_str()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind socket: {}", e);
            process::exit(1);
        }
    };

    println!("Waiting for commands on HTTP {}...", addr);

    // Process all client connections
    for socket in listener.incoming() {
        match socket {
            Ok(socket) => {
                let ctx = ctx.clone();

                thread::spawn(move || {
                    match client(ctx.as_ref(), socket) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    };
                });
            },
            Err(e) => println!("{}", e)
        };
    }
}
