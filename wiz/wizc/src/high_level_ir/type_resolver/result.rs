use crate::high_level_ir::type_resolver::error::ResolverError;
use core::result;

pub type Result<T> = result::Result<T, ResolverError>;
