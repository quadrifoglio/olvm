/*
 * Handlers - Command handling
 */

mod image;
mod vm;

use mongodb::db::Database;

use common::{Result, Error};

/*
 * Handle a command, and return its result as a string
 */
pub fn handle(db: &Database, cmd: &str, obj: &str) -> Result<String> {
    match cmd {
        "createimg" => return image::create(db, obj),
        "listimg" => return image::list(db),
        "getimg" => return image::get(db, obj),
        "updateimg" => return image::update(db, obj),
        "delimg" => return image::delete(db, obj),

        "createvm" => return vm::create(db, obj),
        "listvm" => return vm::list(db),
        "getvm" => return vm::get(db, obj),
        "updatevm" => return vm::update(db, obj),
        "delvm" => return vm::delete(db, obj),

        _ => return Err(Error::new("Unknown command"))
    }
}
