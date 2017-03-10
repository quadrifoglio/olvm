/*
 * VM-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mongodb::db::{Database, ThreadedDatabase};
use bson::{self, Document, Array};

use common::{Result, Error};
use common::structs::VM;

/*
 * Create a new VM in database
 */
pub fn create(db: &Database, vm: &VM) -> Result<()> {
    let doc = try!(vm.to_bson());
    try!(db.collection("vms").insert_one(doc, None));

    Ok(())
}

/*
 * List VMs in database
 */
pub fn list(db: &Database) -> Result<Vec<VM>> {
    let mut vms = Vec::new();
    let cursor = try!(db.collection("vms").find(None, None));

    for result in cursor {
        if let Ok(doc) = result {
            vms.push(try!(VM::from_bson(doc)));
        }
    }

    Ok(vms)
}

/*
 * Get an VM from the database
 */
pub fn get(db: &Database, name: &str) -> Result<VM> {
    let doc = try!(db.collection("vms").find_one(Some(doc!{"name" => name}), None));

    if let Some(vm) = doc {
        return Ok(try!(VM::from_bson(vm)));
    }

    Err(Error::new("VM not found"))
}

/*
 * Store the custom parameters returned by a VM backend script
 */
pub fn params(db: &Database, vm: &mut VM, params: HashMap<String, String>) -> Result<()> {
    if params.len() > 0 {
        for (key, val) in params {
            vm.parameters.insert(key.to_string(), val.to_string());
        }

        try!(update(db, vm));
    }

    Ok(())
}

/*
 * Update an VM in the database
 */
pub fn update(db: &Database, vm: &VM) -> Result<()> {
    let name = vm.name.as_str();

    let mut i = Array::new();
    for iface in &vm.interfaces {
        i.push(bson::to_bson(&iface).unwrap());
    }

    let mut p = Document::new();
    for (k, v) in &vm.parameters {
        p.insert(k.clone(), v.clone());
    }

    let update = doc! {
        "interfaces" => i,
        "parameters" => p
    };

    try!(db.collection("vms").update_one(doc!{"name" => name}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an VM from the database
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(db.collection("vms").delete_one(doc!{"name" => name}, None));
    Ok(())
}
