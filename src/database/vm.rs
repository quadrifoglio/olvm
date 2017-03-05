/*
 * VM-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;

use mongodb::db::Database;

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

impl VM {
    pub fn new() -> VM {
        VM {
            id: 0,
            node: 1, // TODO: Handle node
            backend: 0,
            image: 0,
            name: String::new(),
            parameters: HashMap::new()
        }
    }
}

/*
 * Create a new VM in database
 */
pub fn create(db: &Database, vm: VM) -> Result<i32> {
    Ok(0)
}

/*
 * List VMs in database
 */
pub fn list(db: &Database) -> Result<Vec<VM>> {
    Ok(Vec::new())
}

/*
 * Get a VM from the database
 */
pub fn get(db: &Database, id: i32) -> Result<VM> {
    Ok(VM::new())
}

/*
 * Update a VM in the database
 */
pub fn update(db: &Database, vm: VM) -> Result<()> {
    Ok(())
}

/*
 * Delete a VM from the database
 */
pub fn delete(db: &Database, id: i32) -> Result<()> {
    Ok(())
}
