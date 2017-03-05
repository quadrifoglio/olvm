/*
 * Image-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;
use std::error::Error as StdError;

use bson::{self, Document, Bson};
use mongodb::db::{Database, ThreadedDatabase};

use error::{Result, Error};

/*
 * Data structure to represent an image in database
 */
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub node: i32,
    pub file: String,
    pub parameters: HashMap<String, String>
}

impl Image {
    pub fn new() -> Image {
        Image {
            name: String::new(),
            node: 1, // TODO: Handle node
            file: String::new(),
            parameters: HashMap::new()
        }
    }

    fn from_bson(doc: Document) -> Result<Image> {
        match bson::from_bson::<Image>(Bson::Document(doc)) {
            Ok(img) => Ok(img),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

/*
 * Create a new image in database
 */
pub fn create(db: &Database, img: Image) -> Result<()> {
    let doc = try!(img.to_bson());
    try!(db.collection("images").insert_one(doc, None));

    Ok(())
}

/*
 * List images in database
 */
pub fn list(db: &Database) -> Result<Vec<Image>> {
    let mut imgs = Vec::new();
    let cursor = try!(db.collection("images").find(None, None));

    for result in cursor {
        if let Ok(doc) = result {
            imgs.push(try!(Image::from_bson(doc)));
        }
    }

    Ok(imgs)
}

/*
 * Get an image from the database
 */
pub fn get(db: &Database, name: &str) -> Result<Image> {
    let doc = try!(db.collection("images").find_one(Some(doc!{"name" => name}), None));

    if let Some(img) = doc {
        return Ok(try!(Image::from_bson(img)));
    }

    Err(Error::new("Image not found"))
}

/*
 * Update an image in the database
 */
pub fn update(db: &Database, img: Image) -> Result<()> {
    let name = img.name.as_str();

    try!(db.collection("images").update_one(doc!{"name" => name}, try!(Image::to_bson(&img)), None));
    Ok(())
}

/*
 * Delete an image from the database
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(db.collection("images").delete_one(doc!{"name" => name}, None));
    Ok(())
}
