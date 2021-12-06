use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_use::TypedUse;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedSourceSet {
    File(TypedFile),
    Dir {
        name: String,
        items: Vec<TypedSourceSet>,
    },
}

impl TypedSourceSet {
    fn name(&self) -> &str {
        match self {
            TypedSourceSet::File(f) => &f.name,
            TypedSourceSet::Dir { name, items: _ } => name,
        }
    }
}

impl Ord for TypedSourceSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().cmp(other.name())
    }
}

impl PartialOrd for TypedSourceSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name().partial_cmp(other.name())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedFile {
    pub(crate) name: String,
    pub(crate) uses: Vec<TypedUse>,
    pub(crate) body: Vec<TypedDecl>,
}

impl Ord for TypedFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for TypedFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
