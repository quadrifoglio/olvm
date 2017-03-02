/*
 * Handlers - Command handling
 */

mod image;

use error::{Result, Error};
use parser::Command;

pub fn handle(c: Command) -> Result<()> {
    match c.name.as_str() {
        "createimg" => return image::create(c.parameters),
        _ => return Err(Error::new("Unknown command"))
    }
}
