use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};

#[derive(Debug, Clone)]
pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify(&self, typed_source_set: &TypedSourceSet) {
        match typed_source_set {
            TypedSourceSet::File(f) => self.file(f),
            TypedSourceSet::Dir { name, items } => items.iter().for_each(|i| self.verify(i)),
        }
    }

    fn file(&self, typed_file: &TypedFile) {}
}
