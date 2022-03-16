use wiz_session::Session;
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};

#[derive(Debug)]
pub struct TypeChecker<'s> {
    session: &'s mut Session,
}

impl<'s> TypeChecker<'s> {
    pub fn new(session: &'s mut Session) -> Self {
        Self {session}
    }

    pub fn verify(&self, typed_source_set: &TypedSourceSet) {
        match typed_source_set {
            TypedSourceSet::File(f) => self.file(f),
            TypedSourceSet::Dir { name, items } => items.iter().for_each(|i| self.verify(i)),
        }
    }

    fn file(&self, typed_file: &TypedFile) {}
}
