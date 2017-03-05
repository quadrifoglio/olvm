#[macro_use]
extern crate mongodb;

mod error;
mod parser;
mod backend;
mod database;
mod handler;

use std::io::{self, BufReader, BufRead, Write};

use mongodb::db::Database;

use parser::{Command};
use handler::handle;

fn prompt() {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn command(db: &Database, line: String) {
    if line.len() == 0 {
        prompt();
        return;
    }

    let c = match Command::from_str(line) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            prompt();

            return;
        }
    };

    if let Err(e) = handle(db, c) {
        println!("{}", e);
        prompt();

        return;
    }

    prompt();
}

fn main() {
    let db = match database::open("127.0.0.1", 27017) {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    prompt();

    let r = BufReader::new(io::stdin());
    for line in r.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        command(&db, line);
    }
}
