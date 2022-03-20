mod error;

use crate::high_level_ir::type_checker::error::CheckerError;
use crate::high_level_ir::typed_decl::{
    TypedDecl, TypedExtension, TypedFun, TypedFunBody, TypedProtocol, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt};
use crate::high_level_ir::typed_type::TypedType;
use wiz_session::Session;

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
            TypedSourceSet::Dir { name: _, items } => items.iter().for_each(|i| self.verify(i)),
        }
    }

    fn file(&mut self, typed_file: &TypedFile) {
        typed_file.body.iter().for_each(|d| self.decl(d))
    }

    fn decl(&mut self, decl: &TypedDecl) {
        match decl {
            TypedDecl::Var(v) => self.variable(v),
            TypedDecl::Fun(f) => self.function(f),
            TypedDecl::Struct(s) => self.struct_(s),
            TypedDecl::Class => todo!(),
            TypedDecl::Enum => todo!(),
            TypedDecl::Protocol(p) => self.protocol(p),
            TypedDecl::Extension(e) => self.extension(e),
        }
    }

    fn variable(&mut self, typed_variable: &TypedVar) {
        if typed_variable.type_ != typed_variable.value.type_() {
            self.session.emit_error(CheckerError::new(format!(
                "TypeMissMatchError: left -> {:?}, right -> {:?}",
                typed_variable.type_,
                typed_variable.value.type_()
            )));
        };
        self.expression(&typed_variable.value)
    }

    fn function(&mut self, typed_function: &TypedFun) {
        if let Some(body) = &typed_function.body {
            match body {
                TypedFunBody::Expr(e) => {
                    self.expression(e);
                    if typed_function.return_type != e.type_() {
                        self.session.emit_error(CheckerError::new(format!(
                            "TypeMissMatchError: {:?} excepted return {:?}, but return {:?}",
                            typed_function.name,
                            typed_function.return_type,
                            e.type_(),
                        )));
                    }
                }
                TypedFunBody::Block(b) => {
                    self.block(b);
                }
            };
        }
    }

    fn struct_(&mut self, typed_struct: &TypedStruct) {}

    fn protocol(&mut self, typed_protocol: &TypedProtocol) {}

    fn extension(&mut self, typed_extension: &TypedExtension) {}

    fn statement(&mut self, typed_statement: &TypedStmt) {
        match typed_statement {
            TypedStmt::Expr(e) => self.expression(e),
            TypedStmt::Decl(d) => self.decl(d),
            TypedStmt::Assignment(a) => self.assignment_statement(a),
            TypedStmt::Loop(l) => self.loop_statement(l),
        }
    }

    fn assignment_statement(&mut self, typed_assignment: &TypedAssignmentStmt) {
        match typed_assignment {
            TypedAssignmentStmt::Assignment(a) => {
                if a.target.type_() != a.value.type_() {
                    self.session.emit_error(CheckerError::new(format!(
                        "TypeMissMatchError: assignment {:?}, into {:?}",
                        a.value.type_(),
                        a.target.type_(),
                    )))
                }
            }
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                if a.target.type_() != a.value.type_() {
                    self.session.emit_error(CheckerError::new(format!(
                        "TypeMissMatchError: assignment {:?}, into {:?}",
                        a.value.type_(),
                        a.target.type_(),
                    )))
                }
            }
        }
    }

    fn loop_statement(&mut self, typed_loop_statement: &TypedLoopStmt) {
        match typed_loop_statement {
            TypedLoopStmt::While(w) => {
                if !w
                    .condition
                    .type_()
                    .map(|t| t.is_boolean())
                    .unwrap_or_else(|| false)
                {
                    self.session.emit_error(CheckerError::new(format!(
                        "while condition must be boolean, but {:?}",
                        w.condition.type_()
                    )))
                }
                self.block(&w.block);
            }
            TypedLoopStmt::For(f) => {
                f.iterator.type_();
                self.block(&f.block);
            }
        }
    }

    fn block(&mut self, typed_block: &TypedBlock) {
        typed_block.body.iter().for_each(|s| self.statement(s))
    }

    fn expression(&mut self, typed_expr: &TypedExpr) {}
}
