#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

mod common;
mod parser;
mod database;
mod handler;

fn main() {
    let db = match database::open("127.0.0.1", 27017) {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    parser::stdin::run(&db);
}
