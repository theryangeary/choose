#[derive(Debug)]
pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    ParseRangeError(crate::error::ParseRangeError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseIntError(x) => x.fmt(f),
            ParseError::ParseRangeError(x) => x.fmt(f),
        }
    }
}
