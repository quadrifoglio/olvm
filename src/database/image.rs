/*
 * Image-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use bson::Document;
use mongodb::db::ThreadedDatabase;

use common::{Context, Result, Error};
use common::structs::Image;

/*
 * Create a new image in database
 */
pub fn create(ctx: &Context, img: &Image) -> Result<()> {
    let doc = try!(img.to_bson());
    try!(ctx.db.collection("images").insert_one(doc, None));

    Ok(())
}

/*
 * List images in database
 */
pub fn list(ctx: &Context) -> Result<Vec<Image>> {
    let mut imgs = Vec::new();
    let node = ctx.conf.global.node;
    let cursor = try!(ctx.db.collection("images").find(Some(doc!{"node" => node}), None));

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
pub fn get(ctx: &Context, name: &str) -> Result<Image> {
    let node = ctx.conf.global.node;
    let doc = try!(ctx.db.collection("images").find_one(Some(doc!{"name" => name, "node" => node}), None));

    if let Some(img) = doc {
        return Ok(try!(Image::from_bson(img)));
    }

    Err(Error::new("Image not found"))
}

/*
 * Store the custom parameters returned by an image backend script
 */
pub fn params(ctx: &Context, img: &mut Image, params: HashMap<String, String>) -> Result<()> {
    if params.len() > 0 {
        for (key, val) in params {
            img.parameters.insert(key.to_string(), val.to_string());
        }

        try!(update(ctx, img));
    }

    Ok(())
}

/*
 * Update an image in the database
 */
pub fn update(ctx: &Context, img: &Image) -> Result<()> {
    let node = ctx.conf.global.node;
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

    try!(ctx.db.collection("images").update_one(doc!{"name" => name, "node" => node}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an image from the database
 */
pub fn delete(ctx: &Context, name: &str) -> Result<()> {
    let node = ctx.conf.global.node;
    try!(ctx.db.collection("images").delete_one(doc!{"name" => name, "node" => node}, None));
    Ok(())
}
