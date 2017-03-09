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
pub fn handle(ctx: &Context, cmd: &str, obj: &str) -> Result<String> {
    match cmd {
        "createimg" => return image::create(ctx, obj),
        "listimg" => return image::list(ctx),
        "getimg" => return image::get(ctx, obj),
        "updateimg" => return image::update(ctx, obj),
        "delimg" => return image::delete(ctx, obj),

        "createvm" => return vm::create(ctx, obj),
        "listvm" => return vm::list(ctx),
        "getvm" => return vm::get(ctx, obj),
        "updatevm" => return vm::update(ctx, obj),
        "delvm" => return vm::delete(ctx, obj),
        "startvm" => return vm::start(ctx, obj),
        "stopvm" => return vm::stop(ctx, obj),

        "createnet" => return network::create(ctx, obj),
        "listnet" => return network::list(ctx),
        "getnet" => return network::get(ctx, obj),
        "updatenet" => return network::update(ctx, obj),
        "delnet" => return network::delete(ctx, obj),

        _ => return Err(Error::new("Unknown command"))
    }
}
