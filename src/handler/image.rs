use error::{Result, Error};
use parser::Parameters;

pub fn create(p: Parameters) -> Result<()> {
    let name = try!(p.get("name").ok_or(Error::new("A 'name' parameter is required")));

    println!("creating image with name '{}'", name);

    Ok(())
}
