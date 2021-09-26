use crate::parser::error::ParseError;

pub(crate) type Result<T> = core::result::Result<T, ParseError>;
