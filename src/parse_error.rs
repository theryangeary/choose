use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    ParseRangeError(crate::error::ParseRangeError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ParseIntError(e) => write!(f, "{}", e),
            ParseError::ParseRangeError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ParseError {}
