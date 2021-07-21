use std::error::Error as StdError;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseRange(ParseRangeError),
    TryFromInt(TryFromIntError),
    Config(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(io) => write!(f, "{}", io),
            Self::ParseRange(pr) => write!(f, "{}", pr),
            Self::TryFromInt(tfi) => write!(f, "{}", tfi),
            Self::Config(c) => write!(f, "{}", c),
        }
    }
}

impl StdError for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseRangeError> for Error {
    fn from(e: ParseRangeError) -> Self {
        Self::ParseRange(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromInt(e)
    }
}

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

impl StdError for ParseRangeError {}
