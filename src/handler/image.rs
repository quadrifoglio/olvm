use std::path::Path;

use serde_json::{self};

use common::{Context, Result, Error};
use common::structs::Image;
use database::{self};
use backend;

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(obj: &str) -> Result<Image> {
    let img = try!(Image::from_json(obj));

    if img.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if img.backend.len() == 0 {
        return Err(Error::new("A 'backend' parameter is required"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' is required"));
    }

    // Check if the file exists
    if !Path::new(img.file.as_str()).exists() {
        return Err(Error::new(format!("{}: file not found", img.file)));
    }

    Ok(img)
}

/*
 * Handle a 'createimg' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    let img = try!(validate(&obj));

    try!(database::image::create(&ctx.db, &img));
    try!(backend::image::script_create(ctx, &img));

    Ok(String::new())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(ctx: &Context) -> Result<String> {
    let imgs = try!(database::image::list(&ctx.db));
    let s = try!(serde_json::to_string(&imgs));

    Ok(s)
}

/*
 * Handle a 'getimg' command
 */
pub fn get(ctx: &Context, name: &str) -> Result<String> {
    let img = try!(database::image::get(&ctx.db, name));
    let s = try!(serde_json::to_string(&img));

    Ok(s)
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(ctx: &Context, obj: &str) -> Result<String> {
    let img = try!(validate(&obj));
    try!(database::image::update(&ctx.db, &img));

    Ok(String::new())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    let img = try!(database::image::get(&ctx.db, name));

    try!(backend::image::script_delete(ctx, &img));
    try!(database::image::delete(&ctx.db, img.name.as_str()));

    Ok(String::new())
}
