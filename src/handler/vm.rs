use mongodb::db::Database;
use serde_json::{self};

use common::{Result, Error};
use common::structs::VM;
use database::{self};

/*
 * Validates the user-specified parameters for VM creation
 */
fn validate(db: &Database, obj: &str) -> Result<VM> {
    let vm = try!(VM::from_json(obj));

    if vm.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if vm.backend.len() == 0 {
        return Err(Error::new("A 'backend' is required"));
    }

    // TODO: Check backend, make sure it exists

    if vm.image.len() > 0 {
        if let Err(_) = database::image::get(db, vm.image.as_str()) {
            return Err(Error::new("Image not found"));
        }
    }

    Ok(vm)
}

/*
 * Handle a 'createvm' command
 */
pub fn create(db: &Database, obj: &str) -> Result<String> {
    // Validate and retreive VM info from the client-specified parameters
    let vm = try!(validate(db, &obj));

    if let Ok(_) = database::vm::get(db, vm.name.as_str()) {
        return Err(Error::new("This VM name is not available"));
    }

    // Create the image
    try!(database::vm::create(db, vm));
    Ok(String::new())
}

/*
 * Handle a 'listvm' command
 */
pub fn list(db: &Database) -> Result<String> {
    let vms = try!(database::vm::list(db));
    let s = try!(serde_json::to_string(&vms));

    Ok(s)
}

/*
 * Handle a 'getvm' command
 */
pub fn get(db: &Database, name: &str) -> Result<String> {
    let vm = try!(database::vm::get(db, name));
    let s = try!(serde_json::to_string(&vm));

    Ok(s)
}

/*
 * Handle a 'updatevm' command
 */
pub fn update(db: &Database, obj: &str) -> Result<String> {
    let vm = try!(validate(db, &obj));
    try!(database::vm::update(db, vm));

    Ok(String::new())
}

/*
 * Handle a 'delvm' command
 */
pub fn delete(db: &Database, name: &str) -> Result<String> {
    try!(database::vm::delete(db, name));
    Ok(String::new())
}
