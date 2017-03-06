/*
 * Backend - The actual hypervisors
 */

pub mod image;
pub mod vm;

use std::process::Command;
use std::collections::HashMap;

use common::{Result, Error};

/*
 * Execute a backend script
 */
pub fn script(path: &str, obj: &str) -> Result<HashMap<String, String>> {
    // Execute the script
    println!("started {} {}", path, obj);
    let out = match Command::new(path).arg(obj).output() {
        Ok(out) => out,
        Err(e) => return Err(Error::new(format!("script: exec: {}", e)))
    };
    println!("after");

    // If the script did not exit with status 0, print stderr
    if !out.status.success() {
        let stderr = try!(String::from_utf8(out.stderr).ok().ok_or(Error::new("script: failed to read stderr")));
        return Err(Error::new(format!("script: failed: {}", stderr)));
    }

    // Read stdout to retreive output parameters sent by the script
    let stdout = try!(String::from_utf8(out.stdout).ok().ok_or(Error::new("script: failed to read stdout")));
    let lines = stdout.lines();
    let mut params = HashMap::new();

    for line in lines {
        // Each line should be composed of a key and a value, separated by whitespace(s)
        let mut parts = line.split_whitespace();
        let key = try!(parts.next().ok_or(Error::new("script: invalid output")));
        let value = try!(parts.next().ok_or(Error::new("script: invalid output")));

        params.insert(key.to_string(), value.to_string());
    }

    Ok(params)
}
