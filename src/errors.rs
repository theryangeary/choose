use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseRangeError {
    source_str: String,
}

impl ParseRangeError {
    pub fn new(source_str: &str) -> Self {
        ParseRangeError {
            source_str: String::from(source_str),
        }
    }
}

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source_str)
    }
}

impl Error for ParseRangeError {}
