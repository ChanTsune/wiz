use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_use::TypedUse;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedSourceSet {
    File(TypedFile),
    Dir {
        name: String,
        items: Vec<TypedSourceSet>,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedFile {
    pub(crate) name: String,
    pub(crate) uses: Vec<TypedUse>,
    pub(crate) body: Vec<TypedDecl>,
}
