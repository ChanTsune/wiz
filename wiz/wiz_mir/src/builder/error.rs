use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type BResult<T> = Result<T, BuilderError>;

#[derive(Debug)]
pub struct BuilderError(String);

impl From<String> for BuilderError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for BuilderError {}
