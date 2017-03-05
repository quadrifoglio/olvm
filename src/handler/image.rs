use mongodb::db::Database;

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
pub fn create(db: &Database, obj: &str) -> Result<()> {
    let img = try!(validate(&obj));

    // Check required parameters
    if img.name.len() == 0 {
        return Err(Error::new("A 'name' parameter is required"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' parameter is required"));
    }

    try!(database::image::create(db, img));
    Ok(())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(db: &Database) -> Result<()> {
    let imgs = try!(database::image::list(db));

    for img in imgs {
        println!("name {}, node {}, file {}", img.name, img.node, img.file);
    }

    Ok(())
}

/*
 * Handle a 'getimg' command
 */
pub fn get(db: &Database, name: &str) -> Result<()> {
    let img = try!(database::image::get(db, name));
    println!("name {}, node {}, file {}", img.name, img.node, img.file);

    Ok(())
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(db: &Database, obj: &str) -> Result<()> {
    let img = try!(validate(&obj));
    try!(database::image::update(db, img));

    Ok(())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(database::image::delete(db, name));
    Ok(())
}
