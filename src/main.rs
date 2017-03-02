mod error;
mod parser;

use std::io::{self, BufReader, BufRead, Write};
use parser::{Command};

fn prompt() {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn command(line: String) {
    if line.len() == 0 {
        prompt();
        return;
    }

    let c = Command::from_str(line).unwrap();
    println!("Name: {}", c.name);

    for param in c.parameters {
        println!("Parameter: {} => {}", param.key, param.value);
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
