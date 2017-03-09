#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate toml;
extern crate dhcp;

mod common;
mod config;
mod interface;
mod database;
mod backend;
mod handler;
mod net;

use std::thread;

fn main() {
    // Open and parse configuration file
    let conf = match config::open("/etc/olvm.conf") {
        Ok(conf) => conf,
        Err(e) => {
            println!("Failed to load configuration: {}", e);
            return;
        }
    };

    // Open connection to the database
    let db = match database::open(conf.database.host.as_str(), conf.database.port) {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Create global context, shared everywhere
    let ctx = common::Context {
        conf: conf,
        db: db
    };

    // Start the internal DHCP server
    thread::spawn(|| {
        match net::dhcp::listen() {
            Ok(_) => {},
            Err(e) => println!("Failed to start DHCP server: {}", e)
        };
    });

    // Start the chosen interface, UDP or stdin
    if ctx.conf.udp.is_some() {
        interface::udp::run(&ctx);
    }
    else {
        interface::stdin::run(&ctx);
    }
}
