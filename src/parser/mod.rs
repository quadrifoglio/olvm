use std::result::Result as StdResult;

/*
 * Define a Result type for the parser
 */
pub type Result<T> = StdResult<T, String>;

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
    pub fn from_str(line: &str) -> Result<Parameter> {
        // Split the string by whitespaces
        let line = line.to_string();
        let mut subs = line.split_whitespace();

        // Key first, then value
        let key = try!(subs.next().ok_or("Key not found".to_string()));
        let value = try!(subs.next().ok_or("Value not found".to_string()));

        Ok(Parameter {
            key: key.to_string(),
            value: value.to_string()
        })
    }
}

/*
 * Tests
 */
#[cfg(test)]
mod tests {
    use super::Parameter;

    #[test]
    fn parameter_from_str_valid() {
        let p1 = Parameter::from_str("key1 value1").unwrap();
        let p2 = Parameter::from_str("key2  value2").unwrap();
        let p3 = Parameter::from_str("key3\tvalue3").unwrap();
        let p4 = Parameter::from_str("key4\t \t value4").unwrap();

        assert_eq!(p1.key.as_str(), "key1");
        assert_eq!(p1.value.as_str(), "value1");

        assert_eq!(p2.key.as_str(), "key2");
        assert_eq!(p2.value.as_str(), "value2");

        assert_eq!(p3.key.as_str(), "key3");
        assert_eq!(p3.value.as_str(), "value3");

        assert_eq!(p4.key.as_str(), "key4");
        assert_eq!(p4.value.as_str(), "value4");
    }

    #[test]
    #[should_panic]
    fn parameter_from_str_invalid() {
        Parameter::from_str("key1").unwrap();
        Parameter::from_str("key2\nvalue2").unwrap();
    }
}
