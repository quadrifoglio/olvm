use std::error::Error as StdError;
use std::collections::HashMap;

use bson::{self, Document, Bson};

use common::{Result, Error};

/*
 * Data structure to represent an image
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

    pub fn from_bson(doc: Document) -> Result<Image> {
        match bson::from_bson::<Image>(Bson::Document(doc)) {
            Ok(img) => Ok(img),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

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

    pub fn from_bson(doc: Document) -> Result<VM> {
        match bson::from_bson::<VM>(Bson::Document(doc)) {
            Ok(vm) => Ok(vm),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}
