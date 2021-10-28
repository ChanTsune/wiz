use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct WizError(String);

impl From<String> for WizError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl Display for WizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for WizError {}

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
