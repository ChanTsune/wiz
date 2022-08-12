mod error;

use crate::high_level_ir::type_checker::error::CheckerError;
use wiz_arena::arena::{Arena, StructKind};
use wiz_hir::typed_decl::{
    TypedDeclKind, TypedExtension, TypedFun, TypedFunBody, TypedProtocol, TypedStruct, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedArray, TypedBinOp, TypedCall, TypedExpr, TypedExprKind, TypedIf, TypedInstanceMember,
    TypedLambda, TypedLiteralKind, TypedName, TypedReturn, TypedSubscript, TypedTypeCast,
    TypedUnaryOp,
};
use wiz_hir::typed_file::{TypedFile, TypedSourceSet};
use wiz_hir::typed_stmt::{TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt};
use wiz_hir::typed_type::{Package, TypedType};
use wiz_session::Session;

#[derive(Debug)]
pub struct TypeChecker<'s> {
    session: &'s mut Session,
    arena: &'s Arena,
}

impl<'s> TypeChecker<'s> {
    pub fn new(session: &'s mut Session, arena: &'s Arena) -> Self {
        Self { session, arena }
    }

    pub fn verify(&mut self, typed_source_set: &TypedSourceSet) {
        match typed_source_set {
            TypedSourceSet::File(f) => self.file(f),
            TypedSourceSet::Dir { name: _, items } => items.iter().for_each(|i| self.verify(i)),
        }
    }

    fn file(&mut self, typed_file: &TypedFile) {
        typed_file
            .body
            .iter()
            .for_each(|d| self.decl(&d.kind, &d.package))
    }

    fn decl(&mut self, decl: &TypedDeclKind, package: &Package) {
        match decl {
            TypedDeclKind::Var(v) => self.variable(v),
            TypedDeclKind::Fun(f) => self.function(f),
            TypedDeclKind::Struct(s) => self.struct_(s, package),
            TypedDeclKind::Class => todo!(),
            TypedDeclKind::Enum => todo!(),
            TypedDeclKind::Protocol(p) => self.protocol(p),
            TypedDeclKind::Extension(e) => self.extension(e),
        }
    }

    fn variable(&mut self, typed_variable: &TypedVar) {
        if typed_variable.type_ != typed_variable.value.ty {
            self.session.emit_error(CheckerError::new(format!(
                "TypeMissMatchError: left -> {:?}, right -> {:?}",
                typed_variable.type_, typed_variable.value.ty
            )));
        };
        self.expression(&typed_variable.value)
    }

    fn function(&mut self, typed_function: &TypedFun) {
        if let Some(body) = &typed_function.body {
            match body {
                TypedFunBody::Expr(e) => {
                    self.expression(e);
                    if let Some(expr_type) = &e.ty {
                        if typed_function.return_type != *expr_type {
                            self.session.emit_error(CheckerError::new(format!(
                                "TypeMissMatchError: {:?} excepted return {:?}, but return {:?}",
                                typed_function.name, typed_function.return_type, expr_type,
                            )));
                        }
                    } else {
                        self.session
                            .emit_error(CheckerError::new(format!("Cannot resolve type {:?}", e)))
                    }
                }
                TypedFunBody::Block(b) => {
                    self.block(b);
                }
            };
        }
    }

    fn struct_(&mut self, typed_struct: &TypedStruct, package: &Package) {
        let struct_info = self.arena.get_type(&package.names, &typed_struct.name);

        if let Some(struct_info) = struct_info {
            if struct_info.kind == StructKind::Struct {
                struct_info
                    .conformed_protocols
                    .iter()
                    .for_each(|s| println!("{}: conform {} protocol", typed_struct.name, s))
            } else {
                unreachable!()
            }
        } else {
            self.session.emit_error(CheckerError::new(format!(
                "unknown identifier {}",
                typed_struct.name
            )));
        };
        typed_struct.computed_properties.iter().for_each(|_| {});
        typed_struct.stored_properties.iter().for_each(|_| {});
        typed_struct.member_functions.iter().for_each(|i| {
            if let Some(body) = &i.body {
                match body {
                    TypedFunBody::Expr(e) => self.expression(e),
                    TypedFunBody::Block(b) => self.block(b),
                }
            }
        });
    }

    fn protocol(&mut self, typed_protocol: &TypedProtocol) {}

    fn extension(&mut self, typed_extension: &TypedExtension) {
        typed_extension.computed_properties.iter().for_each(|_| {});
        typed_extension.member_functions.iter().for_each(|i| {
            if let Some(body) = &i.body {
                match body {
                    TypedFunBody::Expr(e) => self.expression(e),
                    TypedFunBody::Block(b) => self.block(b),
                }
            }
        })
    }

    fn statement(&mut self, typed_statement: &TypedStmt) {
        match typed_statement {
            TypedStmt::Expr(e) => self.expression(e),
            TypedStmt::Decl(d) => self.decl(&d.kind, &d.package),
            TypedStmt::Assignment(a) => self.assignment_statement(a),
            TypedStmt::Loop(l) => self.loop_statement(l),
        }
    }

    fn assignment_statement(&mut self, typed_assignment: &TypedAssignmentStmt) {
        match typed_assignment {
            TypedAssignmentStmt::Assignment(a) => {
                if a.target.ty != a.value.ty {
                    self.session.emit_error(CheckerError::new(format!(
                        "TypeMissMatchError: assignment {:?}, into {:?}",
                        a.value.ty, a.target.ty,
                    )))
                }
            }
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                if a.target.ty != a.value.ty {
                    self.session.emit_error(CheckerError::new(format!(
                        "TypeMissMatchError: assignment {:?}, into {:?}",
                        a.value.ty, a.target.ty,
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
                    .ty
                    .as_ref()
                    .map(|t| t.is_boolean())
                    .unwrap_or_else(|| false)
                {
                    self.session.emit_error(CheckerError::new(format!(
                        "while condition must be boolean, but {:?}",
                        w.condition.ty
                    )))
                }
                self.block(&w.block);
            }
            TypedLoopStmt::For(f) => {
                self.block(&f.block);
            }
        }
    }

    fn block(&mut self, typed_block: &TypedBlock) {
        typed_block.body.iter().for_each(|s| self.statement(s))
    }

    fn expression(&mut self, typed_expr: &TypedExpr) {
        let t = &typed_expr.ty;
        match &typed_expr.kind {
            TypedExprKind::Name(n) => self.name(n, t),
            TypedExprKind::Literal(l) => self.literal(l, t),
            TypedExprKind::BinOp(b) => self.binary_operation(b),
            TypedExprKind::UnaryOp(u) => self.unary_operation(u),
            TypedExprKind::Subscript(s) => self.subscript(s),
            TypedExprKind::Member(m) => self.member(m),
            TypedExprKind::Array(a) => self.array(a, t),
            TypedExprKind::Tuple => todo!(),
            TypedExprKind::Dict => todo!(),
            TypedExprKind::StringBuilder => todo!(),
            TypedExprKind::Call(c) => self.call(c),
            TypedExprKind::If(i) => self.if_(i),
            TypedExprKind::When => todo!(),
            TypedExprKind::Lambda(l) => self.lambda(l),
            TypedExprKind::Return(r) => self.return_(r),
            TypedExprKind::TypeCast(c) => self.type_cast(c),
            TypedExprKind::SizeOf(t) => {}
        }
    }

    fn name(&mut self, typed_name: &TypedName, ty: &Option<TypedType>) {
        if ty.is_none() {
            self.session.emit_error(CheckerError::new(format!(
                "Can not resolve name {:?}",
                typed_name.name
            )))
        }
    }

    fn literal(&mut self, typed_literal: &TypedLiteralKind, ty: &Option<TypedType>) {
        match typed_literal {
            TypedLiteralKind::Integer(value) => {
                if let Some(typ) = ty {
                    if !typ.is_integer() {
                        self.session.emit_error(CheckerError::new(format!(
                            "Invalid literal type of {:?}",
                            value
                        )))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!(
                        "Can not resolve literal type of {:?}",
                        value
                    )))
                }
            }
            TypedLiteralKind::FloatingPoint(value) => {
                if let Some(typ) = ty {
                    if !typ.is_floating_point() {
                        self.session.emit_error(CheckerError::new(format!(
                            "Invalid literal type of {:?}",
                            value
                        )))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!(
                        "Can not resolve literal type of {:?}",
                        value
                    )))
                }
            }
            TypedLiteralKind::String(value) => {
                if let Some(typ) = ty {
                    if !typ.is_string_ref() {
                        self.session.emit_error(CheckerError::new(format!(
                            "Invalid literal type of {:?}",
                            value
                        )))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!(
                        "Can not resolve literal type of {:?}",
                        value
                    )))
                }
            }
            TypedLiteralKind::Boolean(value) => {
                if let Some(typ) = ty {
                    if !typ.is_boolean() {
                        self.session.emit_error(CheckerError::new(format!(
                            "Invalid literal type of {:?}",
                            value
                        )))
                    }
                } else {
                    self.session.emit_error(CheckerError::new(format!(
                        "Can not resolve literal type of {:?}",
                        value
                    )))
                }
            }
            TypedLiteralKind::NullLiteral => {
                if ty.is_none() {
                    self.session.emit_error(CheckerError::new(format!(
                        "Can not resolve literal type of null"
                    )))
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
            TypedUnaryOp::Prefix(p) => self.expression(&p.target),
            TypedUnaryOp::Postfix(p) => self.expression(&p.target),
        }
    }

    fn subscript(&mut self, typed_subscript: &TypedSubscript) {
        self.expression(&*typed_subscript.target);
        typed_subscript
            .indexes
            .iter()
            .for_each(|i| self.expression(i))
    }

    fn member(&mut self, typed_member: &TypedInstanceMember) {
        self.expression(&*typed_member.target);
    }

    fn array(&mut self, typed_array: &TypedArray, ty: &Option<TypedType>) {
        typed_array.elements.iter().for_each(|e| {
            self.expression(e);
            if e.ty != *ty {
                self.session.emit_error(CheckerError::new(format!(
                    "TypeMissMatchError: Array element excepted {:?}, but {:?} found",
                    ty, e.ty
                )))
            }
        })
    }

    fn call(&mut self, typed_call: &TypedCall) {
        typed_call
            .args
            .iter()
            .for_each(|a| self.expression(&*a.arg))
    }

    fn if_(&mut self, typed_if: &TypedIf) {
        if typed_if.condition.ty.as_ref().unwrap().is_boolean() {
            self.session.emit_error(CheckerError::new(format!(
                "if condition type must be boolean, but {:?} ware given",
                typed_if.condition.ty
            )))
        }
        self.block(&typed_if.body);
        typed_if.else_body.as_ref().map(|els| self.block(els));
    }

    fn lambda(&mut self, typed_lambda: &TypedLambda) {
        todo!("{:?}", typed_lambda)
    }

    fn return_(&mut self, typed_return: &TypedReturn) {
        typed_return.value.as_ref().map(|v| self.expression(&*v));
    }

    fn type_cast(&mut self, typed_type_cast: &TypedTypeCast) {
        self.expression(&typed_type_cast.target)
    }
}
