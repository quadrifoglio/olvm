/*
 * Network-related database transations
 */

use std::vec::Vec;

use bson::Bson;
use mongodb::db::ThreadedDatabase;

use common::{Context, Result, Error};
use common::structs::Network;

/*
 * Create a new network in database
 */
pub fn create(ctx: &Context, net: &Network) -> Result<()> {
    let doc = try!(net.to_bson());
    try!(ctx.db.collection("networks").insert_one(doc, None));

    Ok(())
}

/*
 * List networks in database
 */
pub fn list(ctx: &Context) -> Result<Vec<Network>> {
    let mut nets = Vec::new();
    let node = ctx.conf.global.node;
    let cursor = try!(ctx.db.collection("networks").find(Some(doc!{"node" => node}), None));

    for result in cursor {
        if let Ok(doc) = result {
            nets.push(try!(Network::from_bson(doc)));
        }
    }

    Ok(nets)
}

/*
 * Get an network from the database
 */
pub fn get(ctx: &Context, name: &str) -> Result<Network> {
    let node = ctx.conf.global.node;
    let doc = try!(ctx.db.collection("networks").find_one(Some(doc!{"name" => name, "node" => node}), None));

    if let Some(net) = doc {
        return Ok(try!(Network::from_bson(net)));
    }

    Err(Error::new("Network not found"))
}

/*
 * Update an network in the database
 */
pub fn update(ctx: &Context, net: &Network) -> Result<()> {
    let node = ctx.conf.global.node;
    let name = net.name.as_str();
    let cidr = net.cidr.as_str();
    let router = net.router.as_str();

    let mut dnsv = Vec::new();
    for d in &net.dns {
        dnsv.push(Bson::String(d.clone()));
    }

    let update = doc! {
        "cidr" => cidr,
        "router" => router,
        "dns" => dnsv
    };

    try!(ctx.db.collection("networks").update_one(doc!{"name" => name, "node" => node}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an network from the database
 */
pub fn delete(ctx: &Context, name: &str) -> Result<()> {
    let node = ctx.conf.global.node;

    try!(ctx.db.collection("networks").delete_one(doc!{"name" => name, "node" => node}, None));
    Ok(())
}
