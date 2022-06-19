use crate::typed_decl::TypedDecl;
use crate::typed_use::TypedUse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedSourceSet {
    File(TypedFile),
    Dir {
        name: String,
        items: Vec<TypedSourceSet>,
    },
}

impl TypedSourceSet {
    pub fn name(&self) -> &str {
        match self {
            TypedSourceSet::File(f) => &f.name,
            TypedSourceSet::Dir { name, items: _ } => name,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedFile {
    pub name: String,
    pub uses: Vec<TypedUse>,
    pub body: Vec<TypedDecl>,
}
