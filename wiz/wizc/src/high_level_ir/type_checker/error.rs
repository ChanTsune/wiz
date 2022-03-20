use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct CheckerError(String);

impl CheckerError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl Display for CheckerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for CheckerError {}
