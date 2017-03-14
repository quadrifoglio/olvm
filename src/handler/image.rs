use std::path::Path;
use std::fs;

use serde_json;

use common::{Context, Result, Error};
use common::structs::Image;
use database;
use backend;

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(ctx: &Context, obj: &str) -> Result<Image> {
    let mut img = try!(Image::from_json(obj));
    img.node = ctx.conf.global.node;

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
    let mut img = try!(validate(ctx, &obj));

    if let Ok(_) = database::image::get(ctx, img.name.as_str()) {
        return Err(Error::new("This image name is not available"));
    }

    let path = try!(ctx.conf.get_image_path(img.backend.as_str(), img.name.as_str()));
    try!(fs::copy(img.file.as_str(), path.as_str()));

    img.file = path;

    try!(database::image::create(ctx, &img));
    try!(backend::image::script_create(ctx, &mut img));

    Ok(String::new())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(ctx: &Context) -> Result<String> {
    let imgs = try!(database::image::list(ctx));
    let s = try!(serde_json::to_string(&imgs));

    Ok(s)
}

/*
 * Handle a 'getimg' command
 */
pub fn get(ctx: &Context, name: &str) -> Result<String> {
    let img = try!(database::image::get(ctx, name));
    let s = try!(serde_json::to_string(&img));

    Ok(s)
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(ctx: &Context, obj: &str) -> Result<String> {
    let img = try!(validate(ctx, &obj));
    try!(database::image::update(ctx, &img));

    Ok(String::new())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(ctx: &Context, name: &str) -> Result<String> {
    let img = try!(database::image::get(ctx, name));

    try!(backend::image::script_delete(ctx, &img));
    try!(database::image::delete(ctx, img.name.as_str()));

    Ok(String::new())
}
