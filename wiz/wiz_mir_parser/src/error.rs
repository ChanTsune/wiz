use std::error::Error;
use std::fmt::{Display, Formatter};

pub type PResult<T> = Result<T, ParseError>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Error for ParseError {}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self(message)
    }
}
