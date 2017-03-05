use mongodb::db::Database;
use serde_json::{self};

use common::{Result, Error};
use common::structs::Image;
use database::{self};

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(obj: &str) -> Result<Image> {
    let img = try!(Image::from_json(obj));

    if img.name.len() == 0 {
        return Err(Error::new("A 'name' is required"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' is required"));
    }

    Ok(img)
}

/*
 * Handle a 'createimg' command
 */
pub fn create(db: &Database, obj: &str) -> Result<String> {
    let img = try!(validate(&obj));

    // Check required parameters
    if img.name.len() == 0 {
        return Err(Error::new("A 'name' parameter is required"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' parameter is required"));
    }

    try!(database::image::create(db, img));
    Ok(String::new())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(db: &Database) -> Result<String> {
    let imgs = try!(database::image::list(db));
    let s = try!(serde_json::to_string(&imgs));

    Ok(s)
}

/*
 * Handle a 'getimg' command
 */
pub fn get(db: &Database, name: &str) -> Result<String> {
    let img = try!(database::image::get(db, name));
    let s = try!(serde_json::to_string(&img));

    Ok(s)
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(db: &Database, obj: &str) -> Result<String> {
    let img = try!(validate(&obj));
    try!(database::image::update(db, img));

    Ok(String::new())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(db: &Database, name: &str) -> Result<String> {
    try!(database::image::delete(db, name));
    Ok(String::new())
}
