use std::fmt::{Debug, Display, Formatter};

use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new<T: ToString>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl StdError for Error {}
