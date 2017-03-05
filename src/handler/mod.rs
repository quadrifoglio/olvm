/*
 * Handlers - Command handling
 */

mod image;
mod vm;

use mongodb::db::Database;

use common::{Result, Error};
use parser::Command;

pub fn handle(db: &Database, c: Command) -> Result<()> {
    match c.name.as_str() {
        "createimg" => return image::create(db, c.parameters),
        "listimg" => return image::list(db),
        "getimg" => return image::get(db, c.parameters),
        "updateimg" => return image::update(db, c.parameters),
        "delimg" => return image::delete(db, c.parameters),

        "createvm" => return vm::create(db, c.parameters),
        "listvm" => return vm::list(db),
        "getvm" => return vm::get(db, c.parameters),
        "updatevm" => return vm::update(db, c.parameters),
        "delvm" => return vm::delete(db, c.parameters),

        _ => return Err(Error::new("Unknown command"))
    }
}
