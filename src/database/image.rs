/*
 * Image-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mysql::PooledConn;

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

/*
 * Create a new image in database and return its ID
 */
pub fn create(db: &mut PooledConn, img: Image) -> Result<i32> {
    // Insert the image
    let id: i32;
    {
        let sql = "INSERT INTO image (ref_node, name, file) VALUES (:a, :b, :c)";
        let stmt = try!(db.prep_exec(sql, params! {
            "a" => 1,
            "b" => img.name,
            "c" => img.file
        }));

        id = stmt.last_insert_id() as i32
    }

    // Insert the custom parameters
    for (key, val) in img.parameters {
        let sql = "INSERT INTO image_param VALUES (:a, :b, :c)";
        try!(db.prep_exec(sql, params! {
            "a" => id,
            "b" => key,
            "c" => val
        }));
    }

    Ok(id)
}

/*
 * List all the images in the database
 */
pub fn list(db: &mut PooledConn) -> Result<Vec<Image>> {
    let mut imgs = Vec::new();
    let rows = try!(db.query("SELECT * FROM image"));

    for row in rows {
        let mut row = try!(row);
        let id: i32 = try!(row.take("id").ok_or(Error::new("Invalid or absent 'id' row")));
        let node: i32 = try!(row.take("ref_node").ok_or(Error::new("Invalid or absent 'ref_node' row")));
        let name: String = try!(row.take("name").ok_or(Error::new("Invalid or absent 'name' row")));
        let file: String = try!(row.take("file").ok_or(Error::new("Invalid or absent 'file' row")));

        imgs.push(Image {
            id: id,
            node: node,
            name: name,
            file: file,
            parameters: HashMap::new()
        });
    }

    Ok(imgs)
}

/*
 * Retreive an image's custom parameters
 */
fn params(db: &mut PooledConn, id: i32) -> Result<HashMap<String, String>> {
    let mut p = HashMap::new();

    let rows = try!(db.prep_exec("SELECT * FROM image_param WHERE ref_img = :a", params! {
        "a" => id
    }));

    for row in rows {
        let mut row = try!(row);
        let key: String = try!(row.take("pkey").ok_or(Error::new("Invalid or absent 'pkey' row")));
        let val: String = try!(row.take("pval").ok_or(Error::new("Invalid or absent 'pval' row")));

        p.insert(key, val);
    }

    Ok(p)
}

/*
 * Get an image's data from the database
 */
pub fn get(db: &mut PooledConn, id: i32) -> Result<Image> {
    let mut img: Image;
    {
        let rows = try!(db.prep_exec("SELECT * FROM image WHERE id = :a", params! {
            "a" => id
        }));

        let mut row = try!(try!(rows.last().ok_or(Error::new("Image for found"))));
        let id: i32 = try!(row.take("id").ok_or(Error::new("Invalid or absent 'id' row")));
        let node: i32 = try!(row.take("ref_node").ok_or(Error::new("Invalid or absent 'ref_node' row")));
        let name: String = try!(row.take("name").ok_or(Error::new("Invalid or absent 'name' row")));
        let file: String = try!(row.take("file").ok_or(Error::new("Invalid or absent 'file' row")));

        img = Image {
            id: id,
            node: node,
            name: name,
            file: file,
            parameters: HashMap::new()
        }
    }

    img.parameters = try!(params(db, id));
    Ok(img)
}

/*
 * Update an image in the database
 */
pub fn update(db: &mut PooledConn, id: i32, name: &str, file: &str) -> Result<()> {
    let sql = "UPDATE image SET name = :a, file = :b WHERE id = :c";

    try!(db.prep_exec(sql, params! {
        "a" => name,
        "b" => file,
        "c" => id
    }));

    Ok(())
}

/*
 * Delete an image from the database
 */
pub fn delete(db: &mut PooledConn, id: i32) -> Result<()> {
    let sql = "DELETE FROM image WHERE id = :a";

    try!(db.prep_exec(sql, params! {
        "a" => id
    }));

    Ok(())
}
