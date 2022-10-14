use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InferError(String);

impl From<&str> for InferError {
    fn from(message: &str) -> Self {
        Self::from(String::from(message))
    }
}

impl From<String> for InferError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl Display for InferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("InferError: ")?;
        f.write_str(&self.0)?;
        f.write_str("\n")
    }
}

impl Error for InferError {}
