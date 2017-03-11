/*
 * Handlers - Command handling
 */

mod image;
mod vm;
mod network;

use common::{Context, Result, Error};

/*
 * Handle a command, and return its result as a string
 */
pub fn handle(ctx: &Context, client: &str, cmd: &str, obj: &str) -> Result<String> {
    let res = match cmd {
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
