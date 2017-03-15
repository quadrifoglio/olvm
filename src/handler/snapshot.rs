use serde_json;

use common::{Context, Result, Error};
use common::structs::Snapshot;
use database;
use backend;

/*
 * Validates the user-specified parameters for snapshot creation/update
 */
fn validate(ctx: &Context, obj: &str) -> Result<Snapshot> {
    let mut snap = try!(Snapshot::from_json(obj));
    snap.node = ctx.conf.global.node;

    if let Err(_) = database::vm::get(ctx, snap.vm.as_str()) {
        return Err(Error::new("VM not found"));
    }

    Ok(snap)
}

/*
 * Handle a 'createsnap' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    let snap = try!(validate(ctx, &obj));

    if let Ok(_) = database::snapshot::get(ctx, snap.vm.as_str(), snap.name.as_str()) {
        return Err(Error::new("This snapshot name is not available"));
    }

    let vm = try!(database::vm::get(ctx, snap.vm.as_str()));

    try!(database::snapshot::create(ctx, &snap));
    try!(backend::vm::script_snapshot_create(ctx, &vm, snap.name.as_str()));

    Ok(String::new())
}

/*
 * Handle a 'listsnap' command
 */
pub fn list(ctx: &Context, vm: &str) -> Result<String> {
    let snap = try!(database::snapshot::list(ctx, vm));
    let s = try!(serde_json::to_string(&snap));

    Ok(s)
}

/*
 * Handle a 'restoresnap' command
 */
pub fn restore(ctx: &Context, obj: &str) -> Result<String> {
    let snap = try!(validate(ctx, &obj));
    let snap = try!(database::snapshot::get(ctx, snap.vm.as_str(), snap.name.as_str()));
    let vm = try!(database::vm::get(ctx, snap.vm.as_str()));

    try!(backend::vm::script_snapshot_restore(ctx, &vm, snap.name.as_str()));

    Ok(String::new())
}

/*
 * Handle a 'delsnap' command
 */
pub fn delete(ctx: &Context, obj: &str) -> Result<String> {
    let snap = try!(validate(ctx, &obj));
    let snap = try!(database::snapshot::get(ctx, snap.vm.as_str(), snap.name.as_str()));
    let vm = try!(database::vm::get(ctx, snap.vm.as_str()));

    try!(backend::vm::script_snapshot_delete(ctx, &vm, snap.name.as_str()));
    try!(database::snapshot::delete(ctx, snap.vm.as_str(), snap.name.as_str()));

    Ok(String::new())
}
