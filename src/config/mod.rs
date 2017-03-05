/*
 * Configuration loading
 */

use std::fs::File;
use std::io::Read;

use common::{Result, Error};

use toml::{self};

/*
 * Database configuration
 */
#[derive(Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16
}

/*
 * UDP interface configuration
 */
#[derive(Deserialize)]
pub struct UDP {
    pub addr: String
}

/*
 * Backend configuration
 */
#[derive(Deserialize)]
pub struct BackendImage {
    pub create: Option<String>,
    pub delete: Option<String>
}

#[derive(Deserialize)]
pub struct BackendVM {
    pub create: Option<String>,
    pub start: Option<String>,
    pub stop: Option<String>,
    pub delete: Option<String>
}

#[derive(Deserialize)]
pub struct Backend {
    pub name: String,
    pub image: BackendImage,
    pub vm: BackendVM
}

/*
 * Configuration file layout
 */
#[derive(Deserialize)]
pub struct Config {
    pub database: Database,
    pub udp: Option<UDP>,
    pub backend: Vec<Backend>
}

impl Config {
    pub fn get_backend(&self, backend: &str) -> Option<&Backend> {
        for b in &self.backend {
            if b.name.as_str() == backend {
                return Some(b)
            }
        }

        None
    }
}

pub fn open(path: &str) -> Result<Config> {
    let mut data = String::new();

    // Open the config file
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(Error::new(format!("open: {}", e)))
    };

    // Read all the content
    match f.read_to_string(&mut data) {
        Ok(_) => {},
        Err(e) => return Err(Error::new(format!("read: {}", e)))
    };

    // Parse the configuration as a toml document
    let conf: Config = match toml::from_str(data.as_str()) {
        Ok(conf) => conf,
        Err(e) => return Err(Error::new(format!("parse: {}", e)))
    };

    Ok(conf)
}
