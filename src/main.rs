#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate toml;

mod common;
mod config;
mod interface;
mod database;
mod backend;
mod handler;

fn main() {
    let conf = match config::open("/etc/olvm.conf") {
        Ok(conf) => conf,
        Err(e) => {
            println!("Failed to load configuration: {}", e);
            return;
        }
    };

    let db = match database::open(conf.database.host.as_str(), conf.database.port) {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    let ctx = common::Context {
        conf: conf,
        db: db
    };

    if ctx.conf.udp.is_some() {
        interface::udp::run(&ctx);
    }
    else {
        interface::stdin::run(&ctx);
    }
}
