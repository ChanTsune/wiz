use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ParseError {
    ParseError(String),
    IOError(io::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseError(e) => f.write_str(e),
            ParseError::IOError(e) => f.write_str(&e.to_string()),
        }
    }
}

impl Error for ParseError {}

impl From<String> for ParseError {
    fn from(str: String) -> Self {
        Self::ParseError(str)
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}
