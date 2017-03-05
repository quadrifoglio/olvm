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
            Ok(l) => l.to_string(),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let (command, obj) = super::parse_command(line);

        match handler::handle(db, command.as_str(), obj.as_str()) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e)
        };

        prompt();
    }
}
