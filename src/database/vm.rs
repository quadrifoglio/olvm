/*
 * VM-related database transations
 */

use std::vec::Vec;
use std::collections::HashMap;
use std::error::Error as StdError;

use bson::{self, Document, Bson};
use mongodb::db::{Database, ThreadedDatabase};

use error::{Result, Error};

/*
 * Data structure to represent a vm in database
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct VM {
    pub name: String,
    pub node: i32,
    pub backend: String,
    pub image: String, // Name of the image the VM is based on (if any)
    pub parameters: HashMap<String, String>
}

impl VM {
    pub fn new() -> VM {
        VM {
            name: String::new(),
            node: 1, // TODO: Handle node
            backend: String::new(),
            image: String::new(),
            parameters: HashMap::new()
        }
    }

    fn from_bson(doc: Document) -> Result<VM> {
        match bson::from_bson::<VM>(Bson::Document(doc)) {
            Ok(vm) => Ok(vm),
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
 * Create a new VM in database
 */
pub fn create(db: &Database, vm: VM) -> Result<()> {
    let doc = try!(vm.to_bson());
    try!(db.collection("vms").insert_one(doc, None));

    Ok(())
}

/*
 * List VMs in database
 */
pub fn list(db: &Database) -> Result<Vec<VM>> {
    let mut vms = Vec::new();
    let cursor = try!(db.collection("vms").find(None, None));

    for result in cursor {
        if let Ok(doc) = result {
            vms.push(try!(VM::from_bson(doc)));
        }
    }

    Ok(vms)
}

/*
 * Get an VM from the database
 */
pub fn get(db: &Database, name: &str) -> Result<VM> {
    let doc = try!(db.collection("vms").find_one(Some(doc!{"name" => name}), None));

    if let Some(vm) = doc {
        return Ok(try!(VM::from_bson(vm)));
    }

    Err(Error::new("VM not found"))
}

/*
 * Update an VM in the database
 */
pub fn update(db: &Database, vm: VM) -> Result<()> {
    let name = vm.name.as_str();

    let mut p = Document::new();
    for (k, v) in &vm.parameters {
        p.insert(k.clone(), v.clone());
    }

    let update = doc! {
        "parameters" => p
    };

    try!(db.collection("vms").update_one(doc!{"name" => name}, doc! {
        "$set" => update
    }, None));

    Ok(())
}

/*
 * Delete an VM from the database
 */
pub fn delete(db: &Database, name: &str) -> Result<()> {
    try!(db.collection("vms").delete_one(doc!{"name" => name}, None));
    Ok(())
}
