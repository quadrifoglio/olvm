#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

mod common;
mod interface;
mod database;
mod handler;

use std::env;

fn main() {
    let mut args = env::args();

    let db = match database::open("127.0.0.1", 27017) {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    if let Some(interface) = args.nth(1) {
        if interface == "udp" {
            interface::udp::run("127.0.0.1:1997", &db);
        }
    }
    else {
        interface::stdin::run(&db);
    }
}
