/*
 * Remote - Send commands to another server
 */

use std::error::Error as StdError;
use std::net::UdpSocket;

use serde_json;
use serde_json::value::Value;

use common::{Result, Error};

/*
 * Send a command to a remote server via UDP
 */
pub fn command(srv: &str, cmd: &str, arg: &str) -> Result<Value> {
    let socket = try!(UdpSocket::bind("0.0.0.0:0"));
    let data = format!("{} {}", cmd, arg);

    try!(socket.send_to(data.as_bytes(), srv));

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, _)) => {
            match String::from_utf8(buf[..len].to_vec()) {
                Ok(s) => {
                    if s.contains("\"error\"") {
                        let json: Value = try!(serde_json::from_str(s.as_str()));
                        let msg = try!(json.get("error").ok_or(Error::new("Remote sent invalid error")));
                        let msg = try!(msg.as_str().ok_or(Error::new("Remote sent invalid error")));

                        Err(Error::new(format!("Remote: {}", msg)))
                    }
                    else {
                        Ok(try!(serde_json::from_str(s.as_str())))
                    }
                }
                Err(_) => Err(Error::new("Invalid response: could not read as a string"))
            }
        },
        Err(e) => Err(Error::new(e.description()))
    }
}
