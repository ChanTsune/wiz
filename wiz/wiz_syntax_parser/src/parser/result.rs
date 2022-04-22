use crate::parser::error::ParseError;

pub type Result<T> = core::result::Result<T, ParseError>;
