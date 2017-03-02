/*
 * Handlers - Command handling
 */

mod image;

use mysql::PooledConn;

use error::{Result, Error};
use parser::Command;

pub fn handle(db: &mut PooledConn, c: Command) -> Result<()> {
    match c.name.as_str() {
        "createimg" => return image::create(db, c.parameters),
        "listimg" => return image::list(db),
        "getimg" => return image::get(db, c.parameters),
        "updateimg" => return image::update(db, c.parameters),
        "delimg" => return image::delete(db, c.parameters),
        _ => return Err(Error::new("Unknown command"))
    }
}
