use serde_json;

use common::{Context, Result, Error};
use common::structs::VM;
use database;
use backend;

/*
 * Validates the user-specified parameters for VM creation
 */
fn validate(ctx: &Context, obj: &str) -> Result<VM> {
    let vm = try!(VM::from_json(obj));

    if vm.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if vm.backend.len() == 0 {
        return Err(Error::new("A 'backend' is required"));
    }

    // TODO: Check backend, make sure it exists

    if vm.image.len() > 0 {
        if let Err(_) = database::image::get(&ctx.db, vm.image.as_str()) {
            return Err(Error::new("Image not found"));
        }
    }

    Ok(vm)
}

/*
 * Handle a 'createvm' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    // Validate and retreive VM info from the client-specified parameters
    let mut vm = try!(validate(ctx, &obj));

    if let Ok(_) = database::vm::get(&ctx.db, vm.name.as_str()) {
        return Err(Error::new("This VM name is not available"));
    }

    // Create the VM
    try!(database::vm::create(&ctx.db, &vm));
    try!(backend::vm::script_create(ctx, &mut vm));

    Ok(String::new())
}

/*
 * Handle a 'listvm' command
 */
pub fn list(ctx: &Context) -> Result<String> {
    let vms = try!(database::vm::list(&ctx.db));
    let s = try!(serde_json::to_string(&vms));

    Ok(s)
}

/*
 * Handle a 'getvm' command
 */
pub fn get(ctx: &Context, name: &str) -> Result<String> {
    let vm = try!(database::vm::get(&ctx.db, name));
    let s = try!(serde_json::to_string(&vm));

    Ok(s)
}

/*
 * Handle a 'updatevm' command
 */
pub fn update(ctx: &Context, obj: &str) -> Result<String> {
    let vm = try!(validate(ctx, &obj));
    try!(database::vm::update(&ctx.db, &vm));

    Ok(String::new())
}

/*
 * Handle a 'delvm' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    let mut vm = try!(database::vm::get(&ctx.db, name));

    try!(database::vm::delete(&ctx.db, name));
    try!(backend::vm::script_delete(ctx, &mut vm));

    Ok(String::new())
}
