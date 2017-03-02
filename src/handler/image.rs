use mysql::PooledConn;

use error::{Result, Error};
use parser::Parameters;

pub fn create(db: &mut PooledConn, p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));
    let file = try!(p.get("file").ok_or(Error::new("A 'file' parameter is required")));

    println!("creating image with name '{}'", name);

    Ok(())
}
