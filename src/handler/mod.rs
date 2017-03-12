/*
 * Handlers - Command handling
 */

mod image;
mod vm;
mod network;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde_json;

use common::{Context, Result, Error};

/*
 * Handle a command, and return its result as a string
 */
pub fn handle(ctx: &Context, client: &str, cmd: &str, obj: &str) -> Result<String> {
    let res = match cmd {
        "status" => status(ctx),

        "createimg" => image::create(ctx, obj),
        "listimg" => image::list(ctx),
        "getimg" => image::get(ctx, obj),
        "updateimg" => image::update(ctx, obj),
        "delimg" => image::delete(ctx, obj),

        "createvm" => vm::create(ctx, obj),
        "listvm" => vm::list(ctx),
        "getvm" => vm::get(ctx, obj),
        "updatevm" => vm::update(ctx, obj),
        "delvm" => vm::delete(ctx, obj),
        "startvm" => vm::start(ctx, obj),
        "stopvm" => vm::stop(ctx, obj),
        "statusvm" => vm::status(ctx, obj),

        "createnet" => network::create(ctx, obj),
        "listnet" => network::list(ctx),
        "getnet" => network::get(ctx, obj),
        "updatenet" => network::update(ctx, obj),
        "delnet" => network::delete(ctx, obj),

        _ => Err(Error::new("Unknown command"))
    };

    match res {
        Ok(s) => {
            println!("[{}]: {}: command executed successfully", client, cmd);
            Ok(s)
        },
        Err(e) => {
            println!("[{}]: {} '{}': error: {}", client, cmd, obj, e);
            Err(e)
        }
    }
}

/*
 * Handle a 'status' command, return information about the host system
 */
fn status(_: &Context) -> Result<String> {
    let mut mem_total = 0.0;
    let mut mem_free = 0.0;
    let mut mem_buffers = 0.0;
    let mut mem_cached = 0.0;

    // Open /proc/meminfo and read it as a string
    let mut f = try!(File::open("/proc/meminfo"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    // Gather memory information
    for line in s.lines() {
        let mut parts = line.split_whitespace();
        let key = try!(parts.next().ok_or(Error::new("Invalid /proc/meminfo")));
        let val = try!(parts.next().ok_or(Error::new("Invalid /proc/meminfo")));

        if key == "MemTotal:" {
            mem_total = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "MemFree:" {
            mem_free = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "Buffers:" {
            mem_buffers = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "Cached:" {
            mem_cached = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
    }

    // Construct a JSON object to return. Values are returned as MiB
    let mut data = HashMap::new();
    data.insert("mem_usage", (mem_total - (mem_free + mem_buffers + mem_cached)) / 1024.0);
    data.insert("mem_total", mem_total / 1024.0);

    Ok(try!(serde_json::to_string(&data)))
}
