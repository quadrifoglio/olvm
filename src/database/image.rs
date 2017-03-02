/*
 * Image-related database transations
 */

use std::vec::Vec;

use mysql::PooledConn;

use error::{Result, Error};

/*
 * Data structure to represent an image in database
 */
pub struct Image {
    pub id: i32,
    pub node: i32,
    pub name: String,
    pub file: String
}

/*
 * Create a new image in database and return it's ID
 */
pub fn create(db: &mut PooledConn, name: &str, file: &str) -> Result<i32> {
    let sql = "INSERT INTO image (ref_node, name, file) VALUES (:a, :b, :c)";
    let stmt = try!(db.prep_exec(sql, params! {
        "a" => 1,
        "b" => name,
        "c" => file
    }));

    Ok(stmt.last_insert_id() as i32)
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
            file: file
        });
    }

    Ok(imgs)
}
