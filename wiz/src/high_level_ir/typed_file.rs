use crate::high_level_ir::typed_decl::TypedDecl;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedFile {
    pub(crate) body: Vec<TypedDecl>,
}
