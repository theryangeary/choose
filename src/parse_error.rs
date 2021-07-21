#[derive(Debug)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    ParseRangeError(crate::error::ParseRangeError),
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            ParseError::ParseIntError(e) => e.to_string(),
            ParseError::ParseRangeError(e) => e.to_string(),
        }
    }
}
