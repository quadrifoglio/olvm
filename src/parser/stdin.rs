use std::io::{self, BufReader, BufRead, Write};

use mongodb::db::Database;

use handler::{self};

/*
 * Print the prompt
 */
fn prompt() {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}

/*
 * Start reading commands from stdin
 */
pub fn run(db: &Database) {
    prompt();

    let r = BufReader::new(io::stdin());
    for line in r.lines() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let command: &str;
        let obj: &str;

        let space = line.find(' ');
        if let Some(space) = space {
            let (c, o) = line.split_at(space);

            command = c.trim();
            obj = o.trim();
        }
        else {
            command = line.as_str();
            obj = "";
        }

        match handler::handle(db, command, obj) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        };

        prompt();
    }
}
