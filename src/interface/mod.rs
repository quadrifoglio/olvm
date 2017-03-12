/*
 * Interface module - Implements the different ways to communicate
 * with the program
 */

pub mod stdin;
pub mod udp;
pub mod http;

/*
 * Parse a command from a string
 * First part is the command name, the second can be either:
 * - an id/name string to identify an object
 * - a JSON string representing an object
 */
pub fn parse_command(s: String) -> (String, String) {
    let command: String;
    let obj: String;

    let s = s.trim().to_string();
    let s = s.replace('\r', "");
    let s = s.replace('\n', "");

    let space = s.find(' ');
    if let Some(space) = space {
        let (c, o) = s.split_at(space);

        command = c.trim().to_string();
        obj = o.trim().to_string();
    }
    else {
        command = s;
        obj = String::new();
    }

    return (command, obj)
}

/*
 * Tests
 */
#[cfg(test)]
mod tests;
