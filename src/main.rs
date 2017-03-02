mod error;
mod parser;
mod handler;

use std::io::{self, BufReader, BufRead, Write};
use parser::{Command};
use handler::handle;

fn prompt() {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn command(line: String) {
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

    if let Err(e) = handle(c) {
        println!("{}", e);
        prompt();

        return;
    }

    prompt();
}

fn main() {
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

        command(line);
    }
}
