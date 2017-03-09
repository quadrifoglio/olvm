/*
 * Network-related database transations
 */

use std::vec::Vec;

use bson::Bson;
use mongodb::db::{Database, ThreadedDatabase};

use common::{Result, Error};
use common::structs::Network;

/*
 * Create a new network in database
 */
pub fn create(db: &Database, net: &Network) -> Result<()> {
    let doc = try!(net.to_bson());
    try!(db.collection("networks").insert_one(doc, None));

    Ok(())
}

/*
 * List networks in database
 */
pub fn list(db: &Database) -> Result<Vec<Network>> {
    let mut nets = Vec::new();
    let cursor = try!(db.collection("networks").find(None, None));

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
pub fn get(db: &Database, name: &str) -> Result<Network> {
    let doc = try!(db.collection("networks").find_one(Some(doc!{"name" => name}), None));

    if let Some(net) = doc {
        return Ok(try!(Network::from_bson(net)));
    }

    Err(Error::new("Network not found"))
}

/*
 * Update an network in the database
 */
pub fn update(db: &Database, net: &Network) -> Result<()> {
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

    try!(db.collection("networks").update_one(doc!{"name" => name}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an network from the database
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(db.collection("networks").delete_one(doc!{"name" => name}, None));
    Ok(())
}
