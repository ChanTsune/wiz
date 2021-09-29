use crate::high_level_ir::typed_decl::TypedDecl;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedSourceSet {
    File(TypedFile),
    Dir {
        name: String,
        items: Vec<TypedSourceSet>
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedFile {
    pub(crate) name: String,
    pub(crate) body: Vec<TypedDecl>,
}
