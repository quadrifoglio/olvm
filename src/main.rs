#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate uuid;
extern crate regex;
extern crate toml;
extern crate dhcp;
extern crate mhttp;

mod utils;
mod common;
mod config;
mod database;
mod interface;
mod backend;
mod remote;
mod handler;
mod net;

use std::thread;
use std::sync::Arc;

fn main() {
    // Open and parse configuration file
    let conf = match config::open("olvm.conf") {
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
    let ctx = Arc::new(common::Context {
        conf: conf,
        db: db
    });

    let rctx = ctx.clone();

    // Setup networking (virtual devices, dhcp server...)
    thread::spawn(move || {
        match net::setup(rctx) {
            Ok(_) => {},
            Err(e) => println!("Failed to setup networking: {}", e)
        };
    });

    // Start the chosen interface, UDP or stdin
    if ctx.conf.udp.is_some() {
        interface::udp::run(&ctx);
    }
    else if ctx.conf.http.is_some() {
        interface::http::run(ctx.clone());
    }
    else {
        interface::stdin::run(&ctx);
    }
}
