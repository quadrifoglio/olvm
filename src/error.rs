use std::fmt::{Display, Debug, Formatter, Result};
use std::error::Error as StdError;

/*
 * Error type
 */
pub struct Error {
    message: String
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into()
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error: {}", self.message)
    }
}
