mod error;

use crate::high_level_ir::type_checker::error::CheckerError;
use crate::high_level_ir::typed_decl::{
    TypedDecl, TypedExtension, TypedFun, TypedFunBody, TypedProtocol, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{TypedArray, TypedBinOp, TypedCall, TypedExpr, TypedIf, TypedInstanceMember, TypedLambda, TypedLiteral, TypedName, TypedReturn, TypedSubscript, TypedTypeCast, TypedUnaryOp};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt};
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

    fn expression(&mut self, typed_expr: &TypedExpr) {
        match typed_expr {
            TypedExpr::Name(n) => {self.name(n)}
            TypedExpr::Literal(l) => {self.literal(l)}
            TypedExpr::BinOp(b) => {self.binary_operation(b)}
            TypedExpr::UnaryOp(u) => {self.unary_operation(u)}
            TypedExpr::Subscript(s) => {self.subscript(s)}
            TypedExpr::Member(m) => {self.member(m)}
            TypedExpr::Array(a) => {self.array(a)}
            TypedExpr::Tuple => todo!(),
            TypedExpr::Dict => todo!(),
            TypedExpr::StringBuilder => todo!(),
            TypedExpr::Call(c) => {self.call(c)}
            TypedExpr::If(i) => {self.if_(i)}
            TypedExpr::When => todo!(),
            TypedExpr::Lambda(l) => {self.lambda(l)}
            TypedExpr::Return(r) => {self.return_(r)}
            TypedExpr::TypeCast(c) => {self.type_cast(c)}
        }
    }

    fn name(&mut self, typed_name: &TypedName) {
        if let None = typed_name.type_ {
            self.session.emit_error(CheckerError::new(format!("Can not resolve name {:?}", typed_name.name)))
        }
    }

    fn literal(&mut self, typed_literal: &TypedLiteral) {
        match typed_literal {
            TypedLiteral::Integer { value, type_ } => {
                if let Some(typ) = type_ {
                    if !typ.is_integer() {
                        self.session.emit_error(CheckerError::new(format!("Invalid literal type of {:?}", value)))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!("Can not resolve literal type of {:?}", value)))
                }
            }
            TypedLiteral::FloatingPoint { value, type_ } => {
                if let Some(typ) = type_ {
                    if !typ.is_floating_point() {
                        self.session.emit_error(CheckerError::new(format!("Invalid literal type of {:?}", value)))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!("Can not resolve literal type of {:?}", value)))
                }
            }
            TypedLiteral::String { value, type_ } => {
                if let Some(typ) = type_ {
                    if !typ.is_string_ref() {
                        self.session.emit_error(CheckerError::new(format!("Invalid literal type of {:?}", value)))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!("Can not resolve literal type of {:?}", value)))
                }
            }
            TypedLiteral::Boolean { value, type_ } => {
                if let Some(typ) = type_ {
                    if !typ.is_boolean() {
                        self.session.emit_error(CheckerError::new(format!("Invalid literal type of {:?}", value)))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!("Can not resolve literal type of {:?}", value)))
                }
            }
            TypedLiteral::NullLiteral { type_ } => {
                if let None = type_ {
                    self.session.emit_error(CheckerError::new(format!("Can not resolve literal type of null")))
                }
            }
        }
    }

    fn binary_operation(&mut self, typed_binop: &TypedBinOp) {
        self.expression(&typed_binop.left);
        self.expression(&typed_binop.right);
    }

    fn unary_operation(&mut self, typed_unop: &TypedUnaryOp) {
        match typed_unop {
            TypedUnaryOp::Prefix(p) => {
                self.expression(&p.target)
            }
            TypedUnaryOp::Postfix(p) => {
                self.expression(&p.target)
            }
        }
    }

    fn subscript(&mut self, typed_subscript: &TypedSubscript) {
        self.expression(&*typed_subscript.target);
        typed_subscript.indexes.iter().for_each(|i|{
            self.expression(i)
        })
    }

    fn member(&mut self, typed_member: &TypedInstanceMember) {
        self.expression(&*typed_member.target);
    }

    fn array(&mut self, typed_array: &TypedArray) {
        typed_array.elements.iter().for_each(|e|{
            self.expression(e);
            if typed_array.type_ != e.type_() {
                self.session.emit_error(CheckerError::new(format!("TypeMissMatchError: Array element excepted {:?}, but {:?} found", typed_array.type_, e.type_())))
            }
        })
    }

    fn call(&mut self, typed_call: &TypedCall) {
        typed_call.args.iter().for_each(|a|{
            self.expression(&*a.arg)
        })
    }

    fn if_(&mut self, typed_if: &TypedIf) {
        if typed_if.condition.type_().unwrap().is_boolean() {
            self.session.emit_error(CheckerError::new(format!("if condition type must be boolean, but {:?} ware given", typed_if.condition.type_())))
        }
        self.block(&typed_if.body);
        typed_if.else_body.as_ref().map(|els|{
            self.block(els)
        });
    }

    fn lambda(&mut self, typed_lambda: &TypedLambda) {
        todo!("{:?}", typed_lambda)
    }

    fn return_(&mut self, typed_return: &TypedReturn) {
        typed_return.value.as_ref().map(|v|{
            self.expression(&*v)
        });
    }

    fn type_cast(&mut self, typed_type_cast: &TypedTypeCast) {
        self.expression(&typed_type_cast.target)
    }
}
