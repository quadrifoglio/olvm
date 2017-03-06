/*
 * Backend  management
 */

use common::{Context, Result, Error};
use common::structs::VM;
use database;

pub fn script_create(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.create {
        let params = try!(super::script(path, try!(vm.to_json()).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_start(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.start {
        let params = try!(super::script(path, try!(vm.to_json()).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_stop(ctx: &Context, vm: &mut VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.stop {
        let params = try!(super::script(path, try!(vm.to_json()).as_str()));
        try!(database::vm::params(&ctx.db, vm, params));
    }

    Ok(())
}

pub fn script_delete(ctx: &Context, vm: &VM) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(vm.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(ref path) = backend.vm.delete {
        try!(super::script(path, try!(vm.to_json()).as_str()));
    }

    Ok(())
}
