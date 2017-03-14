/*
 * VM-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mongodb::db::ThreadedDatabase;
use bson::{self, Document, Array};

use common::{Context, Result, Error};
use common::structs::VM;

/*
 * Create a new VM in database
 */
pub fn create(ctx: &Context, vm: &VM) -> Result<()> {
    let doc = try!(vm.to_bson());
    try!(ctx.db.collection("vms").insert_one(doc, None));

    Ok(())
}

/*
 * List VMs in database
 */
pub fn list(ctx: &Context) -> Result<Vec<VM>> {
    let mut vms = Vec::new();
    let node = ctx.conf.global.node;
    let cursor = try!(ctx.db.collection("vms").find(Some(doc!{"node" => node}), None));

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
pub fn get(ctx: &Context, name: &str) -> Result<VM> {
    let node = ctx.conf.global.node;
    let doc = try!(ctx.db.collection("vms").find_one(Some(doc!{"name" => name, "node" => node}), None));

    if let Some(vm) = doc {
        return Ok(try!(VM::from_bson(vm)));
    }

    Err(Error::new("VM not found"))
}

/*
 * Get a VM by its MAC address
 */
pub fn get_mac(ctx: &Context, mac: &str) -> Result<(VM, usize)> {
    let node = ctx.conf.global.node;
    let cursor = try!(ctx.db.collection("vms").find(Some(doc!{"node" => node}), None));

    for result in cursor {
        if let Ok(doc) = result {
            let vm = try!(VM::from_bson(doc));

            let mut found = false;
            let mut index = 0;

            for iface in &vm.interfaces {
                if iface.mac.as_str() == mac {
                    found = true;
                    break;
                }

                index = index + 1;
            }

            if found {
                return Ok((vm, index));
            }
        }
    }

    Err(Error::new("VM not found"))
}

/*
 * Store the custom parameters returned by a VM backend script
 */
pub fn params(ctx: &Context, vm: &mut VM, params: HashMap<String, String>) -> Result<()> {
    if params.len() > 0 {
        for (key, val) in params {
            vm.parameters.insert(key.to_string(), val.to_string());
        }

        try!(update(ctx, vm));
    }

    Ok(())
}

/*
 * Update an VM in the database
 */
pub fn update(ctx: &Context, vm: &VM) -> Result<()> {
    let node = ctx.conf.global.node;
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

    try!(ctx.db.collection("vms").update_one(doc!{"name" => name, "node" => node}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an VM from the database
 */
pub fn delete(ctx: &Context, name: &str) -> Result<()> {
    let node = ctx.conf.global.node;
    try!(ctx.db.collection("vms").delete_one(doc!{"name" => name, "node" => node}, None));
    Ok(())
}
