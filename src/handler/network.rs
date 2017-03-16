use serde_json;

use common::{Context, Result, Error};
use common::structs::Network;
use database;
use net;

/*
 * Validates the user-specified parameters for network creation/update
 */
fn validate(ctx: &Context, obj: &str) -> Result<Network> {
    let mut net = try!(Network::from_json(obj));
    net.node = ctx.conf.global.node;

    if net.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if net.cidr.len() > 0 {
        if !net::is_valid_cidr(net.cidr.as_str()) {
            return Err(Error::new("Invalid CIDR network address"));
        }
    }
    if net.router.len() > 0 {
        if !net::is_valid_ip(net.router.as_str()) {
            return Err(Error::new("Invalid router IP address"));
        }
    }
    if net.dns.len() > 0 {
        for dns in &net.dns {
            if !net::is_valid_ip(dns) {
                return Err(Error::new("Invalid router IP address"));
            }
        }
    }

    Ok(net)
}

/*
 * Handle a 'createnet' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    let net = try!(validate(ctx, &obj));
    try!(database::network::create(ctx, &net));

    let netname = net::net_dev(net.name.as_str());
    try!(net::system::bridge_create(netname.as_str()));

    if net.interface.len() > 0 {
        try!(net::system::bridge_addif(net.interface.as_str(), netname.as_str()));
    }

    Ok(String::new())
}

/*
 * Handle a 'listnet' command
 */
pub fn list(ctx: &Context) -> Result<String> {
    let nets = try!(database::network::list(ctx));
    let s = try!(serde_json::to_string(&nets));

    Ok(s)
}

/*
 * Handle a 'getnet' command
 */
pub fn get(ctx: &Context, name: &str) -> Result<String> {
    let net = try!(database::network::get(ctx, name));
    let s = try!(serde_json::to_string(&net));

    Ok(s)
}

/*
 * Handle a 'updatenet' command
 */
pub fn update(ctx: &Context, obj: &str) -> Result<String> {
    let net = try!(validate(ctx, &obj));
    try!(database::network::update(ctx, &net));

    Ok(String::new())
}

/*
 * Handle a 'delnet' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    let net = try!(database::network::get(ctx, name));

    try!(database::network::delete(ctx, net.name.as_str()));

    let netname = net::net_dev(net.name.as_str());
    try!(net::system::bridge_delete(netname.as_str()));

    Ok(String::new())
}
