/*
 * Database - Interaction with the data storage system
 */

pub mod image;

use std::error::Error as StdError;

use error::{Error, Result};

use mysql::{Pool};

/*
 * Open a connection pool to the database
 */
pub fn open(user: &str, pass: &str, host: &str, name: &str) -> Result<Pool> {
    match Pool::new(format!("mysql://{}:{}@{}/{}", user, pass, host, name).as_str()) {
        Ok(db) => Ok(db),
        Err(e) => {
            return Err(Error::new(e.description()));
        }
    }
}
