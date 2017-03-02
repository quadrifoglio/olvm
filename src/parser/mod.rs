use std::result::Result as StdResult;
use std::vec::Vec;

use error::Error;

/*
 * Define a Result type for the parser
 */
pub type Result<T> = StdResult<T, Error>;

/*
 * Parameter represents a key-value pair
 */
pub struct Parameter {
    pub key: String,
    pub value: String
}

impl Parameter {
    /*
     * Parse a parameter from a string
     * First is the key, then is an unknown number of whitespaces, and then the value
     */
    pub fn from_str(s: &str) -> Result<Parameter> {
        // Split the string by whitespaces
        let s = s.trim().to_string();
        let mut subs = s.split_whitespace();

        // Key first, then value
        let key = try!(subs.next().ok_or(Error::new("Key not found")));
        let value = try!(subs.next().ok_or(Error::new("Value not found")));

        Ok(Parameter {
            key: key.to_string(),
            value: value.to_string()
        })
    }
}

/*
 * A command is an order adressed to the program to perform an action.
 * It starts by a command name, such as 'createvm' or 'deleteimg' and if followed by
 * a list of parameters
 */
struct Command {
    name: String,
    parameters: Vec<Parameter>
}

impl Command {
    /*
     * Parse a command from a string
     * Valid syntax: <command name> [[, [name] [value]]...]
     */
    fn from_str(s: &str) -> Result<Command> {
        let s = s.trim().to_string();
        if s.len() == 0 {
            return Err(Error::new("Empty command"));
        }

        let name: String;
        let mut parameters = Vec::new();

        // If the command has parameters
        if s.contains(", ") {
            // Split by coma
            let mut subs = s.split(", ");

            // The first substring is the command name
            name = try!(subs.next().ok_or(Error::new("Invalid syntax"))).to_string();

            // Parse parameters
            for param in subs {
                parameters.push(try!(Parameter::from_str(param)));
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
