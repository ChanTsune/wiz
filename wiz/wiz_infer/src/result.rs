use crate::error::InferError;
use core::result;

pub type Result<T> = result::Result<T, InferError>;
