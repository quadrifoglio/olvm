use error::{Result, Error};
use parser::Parameters;
use database::{self};
use database::image::Image;

use mongodb::db::Database;

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(p: &mut Parameters) -> Result<Image> {
    let id = p.get("id");
    let name = p.get("name");
    let file = p.get("file");

    let mut img = Image::new();
    img.parameters = p.clone();

    // Remove 'id' parameter if any
    // This occurs when an update request is performed
    // The 'id' parameter should not be used as an optional parameter
    if let Some(_) = id {
        img.parameters.remove("id");
    }

    // Check name
    if let Some(name) = name {
        img.name = name.clone();
        img.parameters.remove("name");
    }

    // Check image
    if let Some(file) = file {
        img.file = file.clone();
        img.parameters.remove("file");
    }

    Ok(img)
}

/*
 * Handle a 'createimg' command
 */
pub fn create(db: &Database, mut p: Parameters) -> Result<()> {
    let img = try!(validate(&mut p));

    // Check required parameters
    if img.name.len() == 0 {
        return Err(Error::new("A 'name' parameter is required"));
    }
    if img.file.len() == 0 {
        return Err(Error::new("A 'file' parameter is required"));
    }

    let id = try!(database::image::create(db, img));
    println!("id {}", id);

    Ok(())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(db: &Database) -> Result<()> {
    let imgs = try!(database::image::list(db));

    for img in imgs {
        println!("id {}, node {}, name {}, file {}", img.id, img.node, img.name, img.file);
    }

    Ok(())
}

/*
 * Handle a 'getimg' command
 */
pub fn get(db: &Database, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let img = try!(database::image::get(db, id));
    println!("id {}, node {}, name {}, file {}", img.id, img.node, img.name, img.file);

    Ok(())
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(db: &Database, mut p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let mut img = try!(validate(&mut p));
    img.id = id;

    try!(database::image::update(db, img));

    Ok(())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(db: &Database, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    try!(database::image::delete(db, id));

    Ok(())
}
