/*
 * Handlers - Command handling
 */

mod image;
mod vm;
mod network;

use std::collections::HashMap;

use serde_json;

use common::{Context, Result, Error};
use utils;

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
        "migratevm" => vm::migrate(ctx, obj),

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
    // Construct a JSON object to return. Memory values are returned as MiB, and CPU usage as %
    let mut data = HashMap::new();
    let mem = try!(utils::system::global_memory_info());

    data.insert("mem_usage", mem.0);
    data.insert("mem_total", mem.1);
    data.insert("cpu_usage", try!(utils::system::global_cpu_usage()));

    Ok(try!(serde_json::to_string(&data)))
}
