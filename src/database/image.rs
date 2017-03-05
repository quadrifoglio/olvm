/*
 * Image-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mongodb::db::Database;

use error::{Result, Error};

/*
 * Data structure to represent an image in database
 */
pub struct Image {
    pub id: i32,
    pub node: i32,
    pub name: String,
    pub file: String,
    pub parameters: HashMap<String, String>
}

impl Image {
    pub fn new() -> Image {
        Image {
            id: 0,
            node: 1, // TODO: Handle node
            name: String::new(),
            file: String::new(),
            parameters: HashMap::new()
        }
    }
}

/*
 * Create a new image in database
 */
pub fn create(db: &Database, vm: Image) -> Result<i32> {
    Ok(0)
}

/*
 * List images in database
 */
pub fn list(db: &Database) -> Result<Vec<Image>> {
    Ok(Vec::new())
}

/*
 * Get an image from the database
 */
pub fn get(db: &Database, id: i32) -> Result<Image> {
    Ok(Image::new())
}

/*
 * Update an image in the database
 */
pub fn update(db: &Database, vm: Image) -> Result<()> {
    Ok(())
}

/*
 * Delete an image from the database
 */
pub fn delete(db: &Database, id: i32) -> Result<()> {
    Ok(())
}
