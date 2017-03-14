use std::collections::HashMap;

use serde_json;
use serde_json::value::Value;

use common::{Context, Result, Error};
use common::structs::VM;
use database;
use backend;
use net;

/*
 * Validates the user-specified parameters for VM creation, and sets up the VM's network interfaces
 */
fn validate(ctx: &Context, obj: &str) -> Result<VM> {
    let vm = try!(VM::from_json(obj));

    if vm.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if vm.name.len() > 11 {
        return Err(Error::new("The 'name' must be less than 11 characters long"));
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

    let mut index = 0;
    for iface in &vm.interfaces {
        match database::network::get(&ctx.db, iface.network.as_str()) {
            Ok(net) => {
                let ifname = net::iface_dev(vm.name.as_str(), index);
                let netname = net::net_dev(net.name.as_str());

                try!(net::system::tap_create(ifname.as_str()));
                try!(net::system::bridge_addif(ifname.as_str(), netname.as_str()));
            },
            Err(_) => return Err(Error::new(format!("Interface: network '{}' not found", iface.network)))
        };

        index = index + 1;
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

    // Check interfaces and generate MAC addresses
    for iface in &mut vm.interfaces {
        if iface.mac.len() == 0 {
            iface.mac = net::rand_mac();
        }
        else {
            match database::vm::get_mac(&ctx.db, iface.mac.as_str()) {
                Ok(_) => return Err(Error::new("The specified 'mac' address is not available")),
                Err(_) => {}
            };
        }
    }

    // Create the VM
    try!(database::vm::create(&ctx.db, &vm));

    match backend::vm::script_create(ctx, &mut vm) {
        Ok(_) => {},
        Err(e) => {
            let _ = database::vm::delete(&ctx.db, vm.name.as_str());
            return Err(e);
        }
    };

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

    let mut index = 0;
    for _ in &vm.interfaces {
        let ifname = net::iface_dev(vm.name.as_str(), index);
        try!(net::system::tap_delete(ifname.as_str()));

        index = index + 1;
    }

    Ok(String::new())
}

/*
 * Handle a 'startvm' command
 */
pub fn start(ctx: &Context, name: &str) -> Result<String> {
    let mut vm = try!(database::vm::get(&ctx.db, name));

    match backend::vm::script_start(ctx, &mut vm) {
        Ok(_) => {},
        Err(e) => return Err(e)
    };

    Ok(String::new())
}

/*
 * Handle a 'stopvm' command
 */
pub fn stop(ctx: &Context, name: &str) -> Result<String> {
    let mut vm = try!(database::vm::get(&ctx.db, name));

    match backend::vm::script_stop(ctx, &mut vm) {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(e)
    }
}

/*
 * Handle a 'statusvm' command
 */
pub fn status(ctx: &Context, name: &str) -> Result<String> {
    let mut vm = try!(database::vm::get(&ctx.db, name));

    match backend::vm::script_status(ctx, &mut vm) {
        Ok(p) => {
            let mut pp: HashMap<String, Value> = HashMap::new();
            for (k, v) in p {
                if v == "true" {
                    pp.insert(k, Value::Bool(true));
                }
                else if v == "false" {
                    pp.insert(k, Value::Bool(false));
                }
                else {
                    pp.insert(k, Value::String(v));
                }
            }

            Ok(try!(serde_json::to_string(&pp)))
        }
        Err(e) => Err(e)
    }
}
