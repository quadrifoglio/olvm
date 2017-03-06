/*
 * Database - Interaction with the data storage system
 */

pub mod image;
pub mod vm;

use mongodb::Client;
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::ThreadedClient;

use common::Result;

/*
 * Open a connection to the database
 */
pub fn open(host: &str, port: u16) -> Result<Database> {
    let c = try!(Client::connect(host, port));
    let db = c.db("olvm");
    let ver = try!(db.version());

    println!("Connected to MongoDB version {}", ver);

    Ok(db)
}
