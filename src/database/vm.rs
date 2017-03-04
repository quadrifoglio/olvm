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
    pub name: String,
    pub parameters: HashMap<String, String>
}

/*
 * Create a new vm in database and return its ID
 */
pub fn create(db: &mut PooledConn, vm: VM) -> Result<i32> {
    // Insert the vm
    let id: i32;
    {
        let sql = "INSERT INTO vm (ref_node, ref_backend, ref_img, name) VALUES (:a, :b, :c, :d)";
        let stmt = try!(db.prep_exec(sql, params! {
            "a" => 1,
            "b" => vm.backend,
            "c" => vm.image,
            "d" => vm.name
        }));

        id = stmt.last_insert_id() as i32;
    }

    // Insert the custom parameters
    for (key, val) in vm.parameters {
        let sql = "INSERT INTO vm_param VALUES (:a, :b, :c)";
        try!(db.prep_exec(sql, params! {
            "a" => id,
            "b" => key,
            "c" => val
        }));
    }

    // Return the vm's ID
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
            parameters: HashMap::new()
        });
    }

    Ok(imgs)
}

/*
 * Retreive a vm's custom parameters
 */
fn params(db: &mut PooledConn, id: i32) -> Result<HashMap<String, String>> {
    let mut p = HashMap::new();

    let rows = try!(db.prep_exec("SELECT * FROM vm_param WHERE ref_vm = :a", params! {
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
 * Get an vm's data from the database
 */
pub fn get(db: &mut PooledConn, id: i32) -> Result<VM> {
    let mut vm: VM;
    {
        let rows = try!(db.prep_exec("SELECT * FROM vm WHERE id = :a", params! {
            "a" => id
        }));

        let mut row = try!(try!(rows.last().ok_or(Error::new("Failed to fetch row"))));
        let id: i32 = try!(row.take("id").ok_or(Error::new("Invalid or absent 'id' row")));
        let node: i32 = try!(row.take("ref_node").ok_or(Error::new("Invalid or absent 'ref_node' row")));
        let backend: i32 = try!(row.take("ref_backend").ok_or(Error::new("Invalid or absent 'ref_backend' row")));
        let image: i32 = try!(row.take("ref_img").ok_or(Error::new("Invalid or absent 'ref_img' row")));
        let name: String = try!(row.take("name").ok_or(Error::new("Invalid or absent 'name' row")));

        vm = VM {
            id: id,
            node: node,
            backend: backend,
            image: image,
            name: name,
            parameters: HashMap::new()
        };
    }

    vm.parameters = try!(params(db, id));
    Ok(vm)
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
