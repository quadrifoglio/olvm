use mysql::PooledConn;

use error::{Result, Error};
use parser::Parameters;
use database::{self};
use backend::{self};

/*
 * Handle a 'createvm' command
 */
pub fn create(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let backend = try!(p.get("backend").ok_or(Error::new("A 'backend' parameter is required")));
    let image = p.get("image");
    let image_id: i32;
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));

    // Get the backend's ID
    let backend = try!(backend::from_str(backend));

    if let Some(img) = image {
        // Parse the image ID and retreive it from the database
        let image = match img.parse::<i32>() {
            Ok(image) => try!(database::image::get(db, image)),
            Err(_) => return Err(Error::new("The 'image' parameter must be an intger"))
        };

        image_id = image.id;
    }
    else {
        image_id = 0;
    }

    // Create the image
    let id = try!(database::vm::create(db, backend, image_id, name));
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
pub fn update(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let name = p.get("name");
    let mut vm = try!(database::vm::get(db, id));

    if let Some(name) = name {
        vm.name = name.clone()
    }

    try!(database::vm::update(db, id, vm.name.as_str()));

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
