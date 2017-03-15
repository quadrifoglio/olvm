/*
 * Backend  management
 */

use std::error::Error as StdError;
use std::collections::HashMap;

use serde_json;
use serde_json::value::Value;

use common::{Context, Result, Error};
use common::structs::VM;
use database;

/*
 * Convert the VM to a JSON representation
 * This function also adds the proper image definition instead if its name
 */
fn json(ctx: &Context, vm: &VM) -> Result<Value> {
    let mut json = match serde_json::to_value(vm) {
        Ok(json) => json,
        Err(e) => return Err(Error::new(e.description()))
    };

    if vm.image.len() > 0 {
        let img = try!(serde_json::to_value(try!(database::image::get(ctx, vm.image.as_str()))));
        json["image"] = img;
    }

    Ok(json)
}

pub fn script_create(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.create {
        let params = try!(super::script(path, try!(json(ctx, vm)).to_string().as_str()));
        try!(database::vm::params(ctx, vm, params));
    }

    Ok(())
}

pub fn script_start(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.start {
        let params = try!(super::script(path, try!(json(ctx, vm)).to_string().as_str()));
        try!(database::vm::params(ctx, vm, params));
    }

    Ok(())
}

pub fn script_stop(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.stop {
        let params = try!(super::script(path, try!(json(ctx, vm)).to_string().as_str()));
        try!(database::vm::params(ctx, vm, params));
    }

    Ok(())
}

pub fn script_delete(ctx: &Context, vm: &VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.delete {
        try!(super::script(path, try!(json(ctx, vm)).to_string().as_str()));
    }

    Ok(())
}

pub fn script_status(ctx: &Context, vm: &mut VM) -> Result<HashMap<String, String>> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.status {
        return Ok(try!(super::script(path, try!(json(ctx, vm)).to_string().as_str())));
    }

    Ok(HashMap::new())
}

pub fn script_snapshot_create(ctx: &Context, vm: &VM, name: &str) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.snapshot_create {
        let vm_json = try!(json(ctx, vm));
        let json = json!({
            "name": name,
            "vm": vm_json
        }).to_string();

        try!(super::script(path, json.as_str()));
    }

    Ok(())
}

pub fn script_snapshot_restore(ctx: &Context, vm: &VM, name: &str) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.snapshot_restore {
        let vm_json = try!(json(ctx, vm));
        let json = json!({
            "name": name,
            "vm": vm_json
        }).to_string();

        try!(super::script(path, json.as_str()));
    }

    Ok(())
}

pub fn script_snapshot_delete(ctx: &Context, vm: &VM, name: &str) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.snapshot_delete {
        let vm_json = try!(json(ctx, vm));
        let json = json!({
            "name": name,
            "vm": vm_json
        }).to_string();

        try!(super::script(path, json.as_str()));
    }

    Ok(())
}
