use crate::high_level_ir::type_resolver::error::ResolverError;
use core::result;

pub type ResolverResult<T> = result::Result<T, ResolverError>;

