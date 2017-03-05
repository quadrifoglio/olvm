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
 * KVM backend configuration
 */
#[derive(Deserialize)]
pub struct KVMImage {
    pub create: Option<String>,
    pub delete: Option<String>
}

#[derive(Deserialize)]
pub struct KVMVM {
    pub create: Option<String>,
    pub start: Option<String>,
    pub stop: Option<String>,
    pub delete: Option<String>
}

#[derive(Deserialize)]
pub struct KVM {
    pub image: KVMImage,
    pub vm: KVMVM
}

/*
 * Backend configuration
 */
#[derive(Deserialize)]
pub struct Backend {
    pub kvm: Option<KVM>
}

/*
 * Configuration file layout
 */
#[derive(Deserialize)]
pub struct Config {
    pub database: Database,
    pub udp: Option<UDP>,
    pub backend: Backend
}

impl Config {
    pub fn has_backend(&self, backend: &str) -> bool {
        match backend {
            "kvm" => self.backend.kvm.is_some(),
            _ => false
        }
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
