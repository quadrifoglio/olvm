/*
 * Backend  management
 */

use std::error::Error as StdError;

use serde_json;
use mongodb::db::Database;

use common::{Context, Result, Error};
use common::structs::VM;
use database;

/*
 * Convert the VM to a JSON representation
 * This function also adds the proper image definition instead if its name
 */
fn json(db: &Database, vm: &VM) -> Result<String> {
    let mut json = match serde_json::to_value(vm) {
        Ok(json) => json,
        Err(e) => return Err(Error::new(e.description()))
    };

    if vm.image.len() > 0 {
        let img = try!(serde_json::to_value(try!(database::image::get(db, vm.image.as_str()))));
        json["image"] = img;
    }

    match serde_json::to_string(&json) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(e.description()))
    }
}

pub fn script_create(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.create {
        let params = try!(super::script(path, try!(json(&ctx.db, vm)).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_start(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.start {
        let params = try!(super::script(path, try!(json(&ctx.db, vm)).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_stop(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.stop {
        let params = try!(super::script(path, try!(json(&ctx.db, vm)).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_delete(ctx: &Context, vm: &VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.delete {
        try!(super::script(path, try!(json(&ctx.db, vm)).as_str()));
    }

    Ok(())
}
