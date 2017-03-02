#[macro_use]
extern crate mysql;

mod error;
mod parser;
mod backend;
mod database;
mod handler;

use std::io::{self, BufReader, BufRead, Write};

use mysql::PooledConn;

use parser::{Command};
use handler::handle;

fn prompt() {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn command(db: &mut PooledConn, line: String) {
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
    let pool = match database::open("root", "", "127.0.0.1", "olvm") {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return;
        }
    };

    let mut db = match pool.get_conn() {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to get database connection: {}", e);
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

        command(&mut db, line);
    }
}
