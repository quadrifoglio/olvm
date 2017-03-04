use mysql::PooledConn;

use error::{Result, Error};
use parser::Parameters;
use database::{self};
use database::vm::VM;
use backend::{self};

/*
 * Validates the user-specified parameters for VM creation
 */
fn validate(db: &mut PooledConn, p: &mut Parameters) -> Result<VM> {
    let id = p.get("id");
    let backend = p.get("backend");
    let image = p.get("image");
    let name = p.get("name");

    let mut vm = VM::new();
    vm.parameters = p.clone();

    // Remove 'id' parameter if any
    // This occurs when an update request is performed
    // The 'id' parameter should not be used as an optional parameter
    if let Some(_) = id {
        vm.parameters.remove("id");
    }

    // Check backend
    if let Some(backend) = backend {
        vm.backend = try!(backend::from_str(backend));
        vm.parameters.remove("backend");
    }

    // Check image
    if let Some(img) = image {
        // Parse the image ID and retreive it from the database
        let image = match img.parse::<i32>() {
            Ok(image) => try!(database::image::get(db, image)),
            Err(_) => return Err(Error::new("The 'image' parameter must be an intger"))
        };

        vm.image = image.id;
        vm.parameters.remove("image");
    }

    // Check name
    if let Some(name) = name {
        vm.name = name.clone();
        vm.parameters.remove("name");
    }

    Ok(vm)
}

/*
 * Handle a 'createvm' command
 */
pub fn create(db: &mut PooledConn, mut p: Parameters) -> Result<()> {
    // Validate and retreive VM info from the client-specified parameters
    let vm = try!(validate(db, &mut p));

    // Check required parameters
    if vm.backend == 0 {
        return Err(Error::new("A 'backend' parameter is required"));
    }
    if vm.name.len() == 0 {
        return Err(Error::new("A 'name' parameter is required"));
    }

    // Create the image
    let id = try!(database::vm::create(db, vm));
    println!("id {}", id);

    Ok(())
}

/*
 * Handle a 'listvm' command
 */
pub fn list(db: &mut PooledConn) -> Result<()> {
    let vms = try!(database::vm::list(db));

    for vm in vms {
        let backend = try!(backend::to_string(vm.backend));
        println!("id {}, node {}, backend {}, image {}, name {}", vm.id, vm.node, backend, vm.image, vm.name);
    }

    Ok(())
}

/*
 * Handle a 'getvm' command
 */
pub fn get(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let vm = try!(database::vm::get(db, id));
    let backend = try!(backend::to_string(vm.backend));

    println!("id {}, node {}, backend {}, image {}, name {}", vm.id, vm.node, backend, vm.image, vm.name);

    Ok(())
}

/*
 * Handle a 'updatevm' command
 */
pub fn update(db: &mut PooledConn, mut p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let vm = try!(validate(db, &mut p));
    try!(database::vm::update(db, id, vm));

    Ok(())
}

/*
 * Handle a 'delvm' command
 */
pub fn delete(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    try!(database::vm::delete(db, id));

    Ok(())
}
