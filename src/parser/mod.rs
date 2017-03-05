use std::collections::HashMap;

use common::{Result, Error};

/*
 * Parameters are a list of key/value pairs
 */
pub type Parameters = HashMap<String, String>;

/*
 * A command is an order adressed to the program to perform an action.
 * It starts by a command name, such as 'createvm' or 'deleteimg' and if followed by
 * a list of key/value parameters
 */
pub struct Command {
    pub name: String,
    pub parameters: Parameters
}

impl Command {
    /*
     * Parse a command from a string
     * Valid syntax: <command name> [[, [name] [value]]...]
     */
    pub fn from_str<S: Into<String>>(s: S) -> Result<Command> {
        let s = s.into().trim().to_string();
        if s.len() == 0 {
            return Err(Error::new("Empty command"));
        }

        let name: String;
        let mut parameters = HashMap::new();

        // If the command has parameters
        if s.contains(", ") {
            // Split by coma
            let mut subs = s.split(", ");

            // The first substring is the command name
            name = try!(subs.next().ok_or(Error::new("Invalid syntax"))).to_string();

            // Parse parameters
            for param in subs {
                // Split the string by whitespaces
                let s = param.trim().to_string();
                let mut subs = s.split_whitespace();

                // Key first, then value
                let key = try!(subs.next().ok_or(Error::new("Key not found")));
                let value = try!(subs.next().ok_or(Error::new("Value not found")));

                parameters.insert(key.to_string(), value.to_string());
            }
        }
        else {
            name = s;
        }

        Ok(Command {
            name: name,
            parameters: parameters
        })
    }
}

/*
 * Tests
 */
#[cfg(test)]
mod tests;
