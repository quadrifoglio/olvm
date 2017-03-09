use serde_json;

use common::{Context, Result, Error};
use common::structs::Network;
use database;

/*
 * Validates the user-specified parameters for network creation/update
 */
fn validate(obj: &str) -> Result<Network> {
    let mut net = try!(Network::from_json(obj));

    if net.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if net.bridge.len() == 0 {
        net.bridge = format!("net{}", net.name);
    }

    Ok(net)
}

/*
 * Handle a 'createnet' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    let net = try!(validate(&obj));
    try!(database::network::create(&ctx.db, &net));

    Ok(String::new())
}

/*
 * Handle a 'listnet' command
 */
pub fn list(ctx: &Context) -> Result<String> {
    let nets = try!(database::network::list(&ctx.db));
    let s = try!(serde_json::to_string(&nets));

    Ok(s)
}

/*
 * Handle a 'getnet' command
 */
pub fn get(ctx: &Context, name: &str) -> Result<String> {
    let net = try!(database::network::get(&ctx.db, name));
    let s = try!(serde_json::to_string(&net));

    Ok(s)
}

/*
 * Handle a 'updatenet' command
 */
pub fn update(ctx: &Context, obj: &str) -> Result<String> {
    let net = try!(validate(&obj));
    try!(database::network::update(&ctx.db, &net));

    Ok(String::new())
}

/*
 * Handle a 'delnet' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    let net = try!(database::network::get(&ctx.db, name));

    try!(database::network::delete(&ctx.db, net.name.as_str()));

    Ok(String::new())
}
