use mysql::PooledConn;

use error::{Result, Error};
use parser::Parameters;
use database::{self};

/*
 * Handle a 'createimg' command
 */
pub fn create(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    let file = try!(p.get("file").ok_or(Error::new("A 'file' parameter is required")));

    let id = try!(database::image::create(db, name, file));
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
