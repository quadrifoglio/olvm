use serde_json::{self};

use common::{Context, Result, Error};
use common::structs::Image;
use database::{self};

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(ctx: &Context, obj: &str) -> Result<Image> {
    let img = try!(Image::from_json(obj));

    if img.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if img.backend.len() == 0 {
        return Err(Error::new("A 'backend' parameter is required"));
    }
    if !ctx.conf.has_backend(img.backend.as_str()) {
        return Err(Error::new("Invalid or unknown backend"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' is required"));
    }

    Ok(img)
}

/*
 * Handle a 'createimg' command
 */
pub fn create(ctx: &Context, obj: &str) -> Result<String> {
    let img = try!(validate(ctx, &obj));
    try!(database::image::create(&ctx.db, img));

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
    let img = try!(validate(ctx, &obj));
    try!(database::image::update(&ctx.db, img));

    Ok(String::new())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    try!(database::image::delete(&ctx.db, name));
    Ok(String::new())
}
