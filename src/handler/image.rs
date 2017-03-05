use error::{Result, Error};
use parser::Parameters;
use database::{self};
use database::image::Image;

use mongodb::db::Database;

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(p: &mut Parameters) -> Result<Image> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    let file = try!(p.get("file").ok_or(Error::new("A 'file' parameter is required")));

    let mut img = Image::new();
    img.parameters = p.clone();

    img.name = name.to_string();
    img.parameters.remove("name");

    img.file = file.to_string();
    img.parameters.remove("file");

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
pub fn get(db: &Database, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));

    let img = try!(database::image::get(db, name));
    println!("name {}, node {}, file {}", img.name, img.node, img.file);

    Ok(())
}

/*
 * Handle a 'updateimg' command
 */
pub fn update(db: &Database, mut p: Parameters) -> Result<()> {
    let img = try!(validate(&mut p));
    try!(database::image::update(db, img));

    Ok(())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(db: &Database, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    try!(database::image::delete(db, name));

    Ok(())
}
