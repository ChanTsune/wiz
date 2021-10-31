use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct CliError(String);

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        Self(message.to_string())
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for CliError {}

#[derive(Debug)]
pub(crate) struct ProcessError {
    pub(crate) code: Option<i32>,
}

impl ProcessError {
    pub(crate) fn new(code: Option<i32>) -> Self {
        Self { code }
    }
    pub(crate) fn code(code: i32) -> Self {
        Self::new(Some(code))
    }
}

impl Error for ProcessError {}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
