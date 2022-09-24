use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl StdError for Error {}
