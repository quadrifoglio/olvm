/*
 * VM-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mysql::PooledConn;

use error::{Result, Error};

/*
 * Data structure to represent a vm in database
 */
pub struct VM {
    pub id: i32,
    pub node: i32,
    pub backend: i32,
    pub image: i32,
    pub name: String
}

/*
 * Create a new vm in database and return its ID
 */
pub fn create(db: &mut PooledConn, backend: i32, image: i32, name: &str, p: HashMap<String, String>) -> Result<i32> {
    let id: i32;
    {
        let sql = "INSERT INTO vm (ref_node, ref_backend, ref_img, name) VALUES (:a, :b, :c, :d)";
        let stmt = try!(db.prep_exec(sql, params! {
            "a" => 1,
            "b" => backend,
            "c" => image,
            "d" => name
        }));

        id = stmt.last_insert_id() as i32;
    }

    for (key, val) in p {
        // Ignore the required parameters stored in the main table
        if key.as_str() == "backend" || key.as_str() == "image" || key.as_str() == "name" {
            continue;
        }

        let sql = "INSERT INTO vm_param VALUES (:a, :b, :c)";
        try!(db.prep_exec(sql, params! {
            "a" => id,
            "b" => key,
            "c" => val
        }));
    }

    Ok(id)
}

/*
 * List all the vms in the database
 */
pub fn list(db: &mut PooledConn) -> Result<Vec<VM>> {
    let mut imgs = Vec::new();
    let rows = try!(db.query("SELECT * FROM vm"));

    for row in rows {
        let mut row = try!(row);
        let id: i32 = try!(row.take("id").ok_or(Error::new("Invalid or absent 'id' row")));
        let node: i32 = try!(row.take("ref_node").ok_or(Error::new("Invalid or absent 'ref_node' row")));
        let backend: i32 = try!(row.take("ref_backend").ok_or(Error::new("Invalid or absent 'ref_backend' row")));
        let image: i32 = try!(row.take("ref_img").ok_or(Error::new("Invalid or absent 'ref_img' row")));
        let name: String = try!(row.take("name").ok_or(Error::new("Invalid or absent 'name' row")));

        imgs.push(VM {
            id: id,
            node: node,
            backend: backend,
            image: image,
            name: name,
        });
    }

    Ok(imgs)
}

/*
 * Get an vm's data from the database
 */
pub fn get(db: &mut PooledConn, id: i32) -> Result<VM> {
    let rows = try!(db.prep_exec("SELECT * FROM vm WHERE id = :a", params! {
        "a" => id
    }));

    let mut row = try!(try!(rows.last().ok_or(Error::new("Failed to fetch row"))));
    let id: i32 = try!(row.take("id").ok_or(Error::new("Invalid or absent 'id' row")));
    let node: i32 = try!(row.take("ref_node").ok_or(Error::new("Invalid or absent 'ref_node' row")));
    let backend: i32 = try!(row.take("ref_backend").ok_or(Error::new("Invalid or absent 'ref_backend' row")));
    let image: i32 = try!(row.take("ref_img").ok_or(Error::new("Invalid or absent 'ref_img' row")));
    let name: String = try!(row.take("name").ok_or(Error::new("Invalid or absent 'name' row")));

    Ok(VM {
        id: id,
        node: node,
        backend: backend,
        image: image,
        name: name,
    })
}

/*
 * Update an vm in the database
 */
pub fn update(db: &mut PooledConn, id: i32, name: &str) -> Result<()> {
    let sql = "UPDATE vm SET name = :a WHERE id = :b";

    try!(db.prep_exec(sql, params! {
        "a" => name,
        "b" => id,
    }));

    Ok(())
}

/*
 * Delete an vm from the database
 */
pub fn delete(db: &mut PooledConn, id: i32) -> Result<()> {
    let sql = "DELETE FROM vm WHERE id = :a";

    try!(db.prep_exec(sql, params! {
        "a" => id
    }));

    Ok(())
}
