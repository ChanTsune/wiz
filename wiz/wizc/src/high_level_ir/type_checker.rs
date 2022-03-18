use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use wiz_session::Session;
use crate::high_level_ir::type_checker::error::CheckerError;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::typed_decl::{TypedDecl, TypedExtension, TypedFun, TypedProtocol, TypedStruct, TypedVar};
use crate::high_level_ir::typed_stmt::TypedStmt;

#[derive(Debug)]
pub struct TypeChecker<'s> {
    session: &'s mut Session,
}

impl<'s> TypeChecker<'s> {
    pub fn new(session: &'s mut Session) -> Self {
        Self { session }
    }

    pub fn verify(&mut self, typed_source_set: &TypedSourceSet) {
        match typed_source_set {
            TypedSourceSet::File(f) => self.file(f),
            TypedSourceSet::Dir { name, items } => items.iter().for_each(|i| self.verify(i)),
        }
    }

    fn file(&mut self, typed_file: &TypedFile) {
        typed_file.body.iter().for_each(|d|{
            self.decl(d)
        })
    }

    fn decl(&mut self, decl: &TypedDecl) {
        match decl {
            TypedDecl::Var(v) => {self.variable(v)}
            TypedDecl::Fun(f) => {self.function(f)}
            TypedDecl::Struct(s) => {self.struct_(s)}
            TypedDecl::Class => todo!(),
            TypedDecl::Enum => todo!(),
            TypedDecl::Protocol(p) => {self.protocol(p)}
            TypedDecl::Extension(e) => {self.extension(e)}
        }
    }

    fn variable(&mut self, typed_variable: &TypedVar) {

    }

    fn function(&mut self, typed_function: &TypedFun) {

    }

    fn struct_(&mut self, typed_struct: &TypedStruct) {

    }

    fn protocol(&mut self, typed_protocol: &TypedProtocol) {

    }

    fn extension(&mut self, typed_extension: &TypedExtension) {

    }

    fn statement(&mut self, typed_statement: &TypedStmt) {

    }
}
