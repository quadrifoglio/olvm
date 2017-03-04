use mysql::PooledConn;

use error::{Result, Error};
use parser::Parameters;
use database::{self};
use database::image::Image;

/*
 * Validates the user-specified parameters for image creation/update
 */
fn validate(p: &mut Parameters) -> Result<Image> {
    let name = p.get("name");
    let file = p.get("file");

    let mut img = Image::new();
    img.parameters = p.clone();

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
pub fn create(db: &mut PooledConn, mut p: Parameters) -> Result<()> {
    let img = try!(validate(&mut p));

    let id = try!(database::image::create(db, img));
    println!("id {}", id);

    Ok(())
}

/*
 * Handle a 'listimg' command
 */
pub fn list(db: &mut PooledConn) -> Result<()> {
    let imgs = try!(database::image::list(db));

    for img in imgs {
        println!("id {}, node {}, name {}, file {}", img.id, img.node, img.name, img.file);
    }

    Ok(())
}

/*
 * Handle a 'getimg' command
 */
pub fn get(db: &mut PooledConn, p: Parameters) -> Result<()> {
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
pub fn update(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    let name = p.get("name");
    let file = p.get("file");
    let mut img = try!(database::image::get(db, id));

    if let Some(name) = name {
        img.name = name.clone()
    }
    if let Some(file) = file {
        img.file = file.clone()
    }

    try!(database::image::update(db, id, img.name.as_str(), img.file.as_str()));

    Ok(())
}

/*
 * Handle a 'delimg' command
 */
pub fn delete(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let id = try!(p.get("id").ok_or(Error::new("An 'id' parameter is required"))).to_string();
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(Error::new("The 'id' parameter must be an intger"))
    };

    try!(database::image::delete(db, id));

    Ok(())
}
