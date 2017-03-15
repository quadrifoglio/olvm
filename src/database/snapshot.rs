/*
 * Snapshot-related database transations
 */

use std::vec::Vec;

use mongodb::db::ThreadedDatabase;

use common::{Context, Result, Error};
use common::structs::Snapshot;

/*
 * Create a new snapshot in database
 */
pub fn create(ctx: &Context, snap: &Snapshot) -> Result<()> {
    let doc = try!(snap.to_bson());
    try!(ctx.db.collection("snapshots").insert_one(doc, None));

    Ok(())
}

/*
 * List snapshots in database for a given vm
 */
pub fn list(ctx: &Context, vm: &str) -> Result<Vec<Snapshot>> {
    let mut snaps = Vec::new();
    let node = ctx.conf.global.node;
    let cursor = try!(ctx.db.collection("snapshots").find(Some(doc!{"node" => node, "vm" => vm}), None));

    for result in cursor {
        if let Ok(doc) = result {
            snaps.push(try!(Snapshot::from_bson(doc)));
        }
    }

    Ok(snaps)
}

/*
 * Get an snapshot from the database
 */
pub fn get(ctx: &Context, vm: &str, name: &str) -> Result<Snapshot> {
    let node = ctx.conf.global.node;
    let doc = try!(ctx.db.collection("snapshots").find_one(Some(doc!{"name" => name, "vm" => vm, "node" => node}), None));

    if let Some(snap) = doc {
        return Ok(try!(Snapshot::from_bson(snap)));
    }

    Err(Error::new("Snapshot not found"))
}

/*
 * Delete an snapshot from the database
 */
pub fn delete(ctx: &Context, vm: &str, name: &str) -> Result<()> {
    let node = ctx.conf.global.node;
    try!(ctx.db.collection("snapshots").delete_one(doc!{"name" => name, "vm" => vm, "node" => node}, None));
    Ok(())
}
