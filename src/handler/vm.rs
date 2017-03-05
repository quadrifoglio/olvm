use error::{Result, Error};
use parser::Parameters;
use database::{self};
use database::vm::VM;

use mongodb::db::Database;

/*
 * Validates the user-specified parameters for VM creation
 */
fn validate(db: &Database, p: &mut Parameters) -> Result<VM> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    let backend = try!(p.get("backend").ok_or(Error::new("A 'backend' parameter is required")));
    let image = p.get("image");

    let mut vm = VM::new();
    vm.parameters = p.clone();

    vm.name = name.to_string();
    vm.parameters.remove("name");

    // TODO: Check backend, make sure it exists
    vm.backend = backend.to_string();
    vm.parameters.remove("backend");

    if let Some(img) = image {
        if let Ok(_) = database::image::get(db, img) {
            vm.image = img.to_string();
        }
        else {
            return Err(Error::new("Image not found"));
        }

        vm.parameters.remove("image");
    }

    Ok(vm)
}

/*
 * Handle a 'createvm' command
 */
pub fn create(db: &Database, mut p: Parameters) -> Result<()> {
    // Validate and retreive VM info from the client-specified parameters
    let vm = try!(validate(db, &mut p));

    if let Ok(_) = database::vm::get(db, vm.name.as_str()) {
        return Err(Error::new("This VM name is not available"));
    }

    // Create the image
    try!(database::vm::create(db, vm));
    Ok(())
}

/*
 * Handle a 'listvm' command
 */
pub fn list(db: &Database) -> Result<()> {
    let vms = try!(database::vm::list(db));

    for vm in vms {
        println!("name {}, node {}, backend {}, image {}", vm.name, vm.node, vm.backend, vm.image);
    }

    Ok(())
}

/*
 * Handle a 'getvm' command
 */
pub fn get(db: &Database, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));

    let vm = try!(database::vm::get(db, name));
    println!("name {}, node {}, backend {}, image {}", vm.name, vm.node, vm.backend, vm.image);

    Ok(())
}

/*
 * Handle a 'updatevm' command
 */
pub fn update(db: &Database, mut p: Parameters) -> Result<()> {
    let vm = try!(validate(db, &mut p));
    try!(database::vm::update(db, vm));

    Ok(())
}

/*
 * Handle a 'delvm' command
 */
pub fn delete(db: &Database, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    try!(database::vm::delete(db, name));

    Ok(())
}
