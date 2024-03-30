use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    ParseRangeError(crate::error::ParseRangeError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseIntError(e) => write!(f, "{}", e),
            ParseError::ParseRangeError(e) => write!(f, "{}", e),
        }
    }
}
