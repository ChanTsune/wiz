use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ResolverError {
    message: String,
}

impl From<&str> for ResolverError {
    fn from(message: &str) -> Self {
        Self::from(String::from(message))
    }
}

impl From<String> for ResolverError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ResolverError: ")?;
        f.write_str(&self.message)?;
        f.write_str("\n")
    }
}

impl Error for ResolverError {}
