/*
 * Image-related database transations
 */

use std::vec::Vec;

use bson::Document;
use mongodb::db::{Database, ThreadedDatabase};

use common::{Result, Error};
use common::structs::Image;

/*
 * Create a new image in database
 */
pub fn create(db: &Database, img: &Image) -> Result<()> {
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
pub fn update(db: &Database, img: &Image) -> Result<()> {
    let name = img.name.as_str();
    let file = img.file.as_str();

    let mut p = Document::new();
    for (k, v) in &img.parameters {
        p.insert(k.clone(), v.clone());
    }

    let update = doc! {
        "file" => file,
        "parameters" => p
    };

    try!(db.collection("images").update_one(doc!{"name" => name}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an image from the database
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(db.collection("images").delete_one(doc!{"name" => name}, None));
    Ok(())
}
