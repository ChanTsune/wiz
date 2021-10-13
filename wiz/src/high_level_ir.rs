use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDecl, TypedFun, TypedFunBody, TypedInitializer,
    TypedMemberFunction, TypedStoredProperty, TypedStruct, TypedUse, TypedValueArgDef, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedArray, TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember,
    TypedLambda, TypedLiteral, TypedName, TypedPostfixUnaryOp, TypedPrefixUnaryOp, TypedReturn,
    TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedType, TypedTypeParam, TypedValueType};
use crate::syntax::annotation::AnnotationsSyntax;
use crate::syntax::block::BlockSyntax;
use crate::syntax::decl::{
    Decl, FunSyntax, InitializerSyntax, MethodSyntax, StoredPropertySyntax, StructPropertySyntax,
    StructSyntax, VarSyntax,
};
use crate::syntax::expr::{
    ArraySyntax, BinaryOperationSyntax, CallExprSyntax, Expr, IfExprSyntax, LambdaSyntax,
    MemberSyntax, NameExprSyntax, PostfixUnaryOperationSyntax, PrefixUnaryOperationSyntax,
    ReturnSyntax, SubscriptSyntax, TypeCastSyntax, UnaryOperationSyntax,
};
use crate::syntax::file::{FileSyntax, SourceSet, WizFile};
use crate::syntax::fun::arg_def::ArgDef;
use crate::syntax::fun::body_def::FunBody;
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::stmt::{AssignmentStmt, LoopStmt, Stmt, WhileLoopSyntax};
use crate::syntax::type_name::{TypeName, TypeParam};
use crate::utils::path_string_to_page_name;
use std::option::Option::Some;

pub mod type_resolver;
pub mod typed_annotation;
pub mod typed_decl;
pub mod typed_expr;
pub mod typed_file;
pub mod typed_stmt;
pub mod typed_type;

pub struct Ast2HLIR;

impl Ast2HLIR {
    pub fn new() -> Self {
        Self {}
    }

    pub fn source_set(&mut self, s: SourceSet) -> TypedSourceSet {
        match s {
            SourceSet::File(f) => TypedSourceSet::File(self.file(f)),
            SourceSet::Dir { name, items } => TypedSourceSet::Dir {
                name,
                items: items.into_iter().map(|i| self.source_set(i)).collect(),
            },
        }
    }

    pub fn file(&mut self, f: WizFile) -> TypedFile {
        TypedFile {
            name: path_string_to_page_name(f.name),
            body: self.file_syntax(f.syntax),
        }
    }

    pub fn file_syntax(&mut self, f: FileSyntax) -> Vec<TypedDecl> {
        f.body.into_iter().map(|d| self.decl(d)).collect()
    }

    pub(crate) fn annotations(&self, a: Option<AnnotationsSyntax>) -> TypedAnnotations {
        match a {
            None => TypedAnnotations::new(),
            Some(a) => TypedAnnotations::from(
                a.annotations
                    .into_iter()
                    .map(|a| a.name.token)
                    .collect::<Vec<String>>(),
            ),
        }
    }

    pub fn stmt(&self, s: Stmt) -> TypedStmt {
        match s {
            Stmt::Decl(decl) => TypedStmt::Decl(self.decl(decl)),
            Stmt::Expr(expr) => TypedStmt::Expr(self.expr(expr)),
            Stmt::Assignment(a) => TypedStmt::Assignment(self.assignment(a)),
            Stmt::Loop(l) => TypedStmt::Loop(self.loop_stmt(l)),
        }
    }

    pub fn assignment(&self, a: AssignmentStmt) -> TypedAssignmentStmt {
        match a {
            AssignmentStmt::Assignment(a) => TypedAssignmentStmt::Assignment(TypedAssignment {
                target: self.expr(a.target),
                value: self.expr(a.value),
            }),
            AssignmentStmt::AssignmentAndOperator(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(TypedAssignmentAndOperation {
                    target: self.expr(a.target),
                    operator: a.operator,
                    value: self.expr(a.value),
                })
            }
        }
    }

    pub fn loop_stmt(&self, l: LoopStmt) -> TypedLoopStmt {
        match l {
            LoopStmt::While(WhileLoopSyntax { condition, block }) => {
                TypedLoopStmt::While(TypedWhileLoopStmt {
                    condition: self.expr(condition),
                    block: self.block(block),
                })
            }
            LoopStmt::For {
                values,
                iterator,
                block,
            } => TypedLoopStmt::For(TypedForStmt {
                values,
                iterator: self.expr(iterator),
                block: self.block(block),
            }),
        }
    }

    pub fn decl(&self, d: Decl) -> TypedDecl {
        match d {
            Decl::Var(v) => TypedDecl::Var(self.var_syntax(v)),
            Decl::Fun(f) => TypedDecl::Fun(self.fun_syntax(f)),
            Decl::Struct(s) => {
                let struct_ = self.struct_syntax(s);
                let struct_ = self.default_init_if_needed(struct_);
                TypedDecl::Struct(struct_)
            }
            Decl::ExternC { .. } => TypedDecl::Class,
            Decl::Enum { .. } => TypedDecl::Enum,
            Decl::Protocol { .. } => TypedDecl::Protocol,
            Decl::Extension { .. } => TypedDecl::Extension,
            Decl::Use(u) => TypedDecl::Use(TypedUse {
                annotations: self.annotations(u.annotations),
                package: Package {
                    names: u.package_name.names,
                },
                alias: u.alias,
            }),
        }
    }

    pub fn var_syntax(&self, v: VarSyntax) -> TypedVar {
        let expr = self.expr(v.value);
        TypedVar {
            annotations: self.annotations(v.annotations),
            package: None,
            is_mut: v.is_mut,
            name: v.name,
            type_: None,
            value: expr,
        }
    }

    pub fn arg_def(&self, a: ArgDef) -> TypedArgDef {
        match a {
            ArgDef::Value(a) => TypedArgDef::Value(TypedValueArgDef {
                label: a.label,
                name: a.name,
                type_: self.type_(a.type_name),
            }),
            ArgDef::Self_(s) => match s.reference {
                None => TypedArgDef::Self_(None),
                Some(_) => TypedArgDef::RefSelf(None),
            },
        }
    }

    pub fn fun_body(&self, body: FunBody) -> TypedFunBody {
        match body {
            FunBody::Block { block } => TypedFunBody::Block(self.block(block)),
            FunBody::Expr { expr } => TypedFunBody::Expr(self.expr(expr)),
        }
    }

    pub fn fun_syntax(&self, f: FunSyntax) -> TypedFun {
        let args: Vec<TypedArgDef> = f.arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        let body = match f.body {
            None => None,
            Some(b) => Some(self.fun_body(b)),
        };

        TypedFun {
            annotations: self.annotations(f.annotations),
            package: None,
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params.map(|v| {
                v.into_iter()
                    .map(|p| TypedTypeParam {
                        name: p.name,
                        type_constraint: match p.type_constraints {
                            None => {
                                vec![]
                            }
                            Some(tn) => {
                                vec![self.type_(tn)]
                            }
                        },
                    })
                    .collect()
            }),
            arg_defs: args,
            body: body,
            return_type: match f.return_type {
                Some(type_name) => Some(self.type_(type_name)),
                None => None,
            },
        }
    }

    pub fn type_(&self, tn: TypeName) -> TypedType {
        match tn {
            TypeName::Simple(stn) => TypedType::Value(TypedValueType {
                package: Some(Package::global()),
                name: stn.name,
                type_args: stn
                    .type_args
                    .map(|v| v.into_iter().map(|t| self.type_(t)).collect()),
            }),
            TypeName::Decorated(d) => {
                if d.decoration == "&" {
                    let t = self.type_(d.type_);
                    match t {
                        TypedType::Value(v) => TypedType::Reference(v),
                        TypedType::Function(_) => {
                            todo!()
                        }
                        TypedType::Type(_) => {
                            todo!()
                        }
                        TypedType::Reference(_) => {
                            panic!("Reference can not reference.")
                        }
                    }
                } else {
                    todo!()
                }
            }
        }
    }

    fn type_param(&self, tp: TypeParam) -> TypedTypeParam {
        TypedTypeParam {
            name: tp.name,
            type_constraint: tp.type_constraints.map_or(vec![], |v| vec![self.type_(v)]),
        }
    }

    pub fn struct_syntax(&self, s: StructSyntax) -> TypedStruct {
        let mut stored_properties: Vec<TypedStoredProperty> = vec![];
        let mut computed_properties: Vec<TypedComputedProperty> = vec![];
        let mut initializers: Vec<TypedInitializer> = vec![];
        let mut member_functions: Vec<TypedMemberFunction> = vec![];
        for p in s.properties {
            match p {
                StructPropertySyntax::StoredProperty(v) => {
                    stored_properties.push(self.stored_property_syntax(v));
                }
                StructPropertySyntax::ComputedProperty => {}
                StructPropertySyntax::Init(init) => {
                    initializers.push(self.initializer_syntax(init))
                }
                StructPropertySyntax::Method(method) => {
                    member_functions.push(self.member_function(method))
                }
                StructPropertySyntax::Deinit(deinit) => {
                    todo!("deinit {:?}", deinit)
                }
            };
        }
        TypedStruct {
            annotations: self.annotations(s.annotations),
            package: None,
            name: s.name,
            type_params: s
                .type_params
                .map(|v| v.into_iter().map(|tp| self.type_param(tp)).collect()),
            init: initializers,
            stored_properties,
            computed_properties,
            member_functions,
            static_function: vec![],
        }
    }

    fn default_init_if_needed(&self, mut s: TypedStruct) -> TypedStruct {
        let args: Vec<TypedArgDef> = s
            .stored_properties
            .iter()
            .map(|p| {
                TypedArgDef::Value(TypedValueArgDef {
                    label: p.name.clone(),
                    name: p.name.clone(),
                    type_: p.type_.clone(),
                })
            })
            .collect();
        if s.init.is_empty() {
            let struct_type = TypedValueType {
                package: Some(Package::global()),
                name: s.name.clone(),
                type_args: None,
            };
            s.init.push(TypedInitializer {
                args,
                body: TypedFunBody::Block(TypedBlock {
                    body: s
                        .stored_properties
                        .iter()
                        .map(|p| {
                            TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            package: None,
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(struct_type.clone())),
                                        })),
                                        name: p.name.clone(),
                                        is_safe: false,
                                        type_: Some(p.type_.clone()),
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: None,
                                        name: p.name.clone(),
                                        type_: Some(p.type_.clone()),
                                    }),
                                },
                            ))
                        })
                        .collect(),
                }),
            })
        }
        s
    }

    pub fn stored_property_syntax(&self, p: StoredPropertySyntax) -> TypedStoredProperty {
        TypedStoredProperty {
            name: p.name,
            type_: self.type_(p.type_),
        }
    }

    pub fn initializer_syntax(&self, init: InitializerSyntax) -> TypedInitializer {
        TypedInitializer {
            args: init.args.into_iter().map(|a| self.arg_def(a)).collect(),
            body: self.fun_body(init.body),
        }
    }

    pub fn member_function(&self, member_function: MethodSyntax) -> TypedMemberFunction {
        let MethodSyntax {
            name,
            args,
            type_params,
            body,
            return_type,
        } = member_function;

        let rt = return_type.map(|r| self.type_(r));
        let fb = body.map(|b| self.fun_body(b));
        TypedMemberFunction {
            name: name,
            args: args.into_iter().map(|a| self.arg_def(a)).collect(),
            type_params: type_params
                .map(|tps| tps.into_iter().map(|p| self.type_param(p)).collect()),
            body: fb,
            return_type: rt,
        }
    }

    pub fn expr(&self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name(n) => TypedExpr::Name(self.name_syntax(n)),
            Expr::Literal(literal) => TypedExpr::Literal(self.literal_syntax(literal)),
            Expr::BinOp(b) => TypedExpr::BinOp(self.binary_operation_syntax(b)),
            Expr::UnaryOp(u) => TypedExpr::UnaryOp(self.unary_operation_syntax(u)),
            Expr::Subscript(s) => TypedExpr::Subscript(self.subscript_syntax(s)),
            Expr::Member(m) => TypedExpr::Member(self.member_syntax(m)),
            Expr::Array(a) => TypedExpr::Array(self.array_syntax(a)),
            Expr::Tuple { .. } => TypedExpr::Tuple,
            Expr::Dict { .. } => TypedExpr::Dict,
            Expr::StringBuilder { .. } => TypedExpr::StringBuilder,
            Expr::Call(c) => TypedExpr::Call(self.call_syntax(c)),
            Expr::If(i) => TypedExpr::If(self.if_syntax(i)),
            Expr::When { .. } => TypedExpr::When,
            Expr::Lambda(l) => TypedExpr::Lambda(self.lambda_syntax(l)),
            Expr::Return(r) => TypedExpr::Return(self.return_syntax(r)),
            Expr::TypeCast(t) => TypedExpr::TypeCast(self.type_cast(t)),
        }
    }

    pub fn literal_syntax(&self, literal: LiteralSyntax) -> TypedLiteral {
        match literal {
            LiteralSyntax::Integer(value) => TypedLiteral::Integer {
                value: value.token,
                type_: Some(TypedType::int64()),
            },
            LiteralSyntax::FloatingPoint(value) => TypedLiteral::FloatingPoint {
                value: value.token,
                type_: Some(TypedType::double()),
            },
            LiteralSyntax::String {
                open_quote: _,
                value,
                close_quote: _,
            } => TypedLiteral::String {
                value,
                type_: Some(TypedType::string()),
            },
            LiteralSyntax::Boolean(syntax) => TypedLiteral::Boolean {
                value: syntax.token,
                type_: Some(TypedType::bool()),
            },
            LiteralSyntax::Null => TypedLiteral::NullLiteral { type_: None },
        }
    }

    pub fn name_syntax(&self, n: NameExprSyntax) -> TypedName {
        let NameExprSyntax { name_space, name } = n;
        TypedName {
            package: if name_space.is_empty() {
                None
            } else {
                Some(Package::new(name_space))
            },
            name,
            type_: None,
        }
    }

    pub fn binary_operation_syntax(&self, b: BinaryOperationSyntax) -> TypedBinOp {
        let BinaryOperationSyntax {
            left,
            operator: kind,
            right,
        } = b;
        let left = Box::new(self.expr(*left));
        let right = Box::new(self.expr(*right));
        TypedBinOp {
            left,
            kind: kind.token,
            right,
            type_: None,
        }
    }

    pub fn unary_operation_syntax(&self, u: UnaryOperationSyntax) -> TypedUnaryOp {
        match u {
            UnaryOperationSyntax::Prefix(p) => {
                TypedUnaryOp::Prefix(self.prefix_unary_operation_syntax(p))
            }
            UnaryOperationSyntax::Postfix(p) => {
                TypedUnaryOp::Postfix(self.postfix_unary_operation_syntax(p))
            }
        }
    }

    pub fn prefix_unary_operation_syntax(
        &self,
        p: PrefixUnaryOperationSyntax,
    ) -> TypedPrefixUnaryOp {
        let PrefixUnaryOperationSyntax { operator, target } = p;
        let target = self.expr(*target);
        TypedPrefixUnaryOp {
            target: Box::new(target),
            kind: operator.token,
            type_: None,
        }
    }

    pub fn postfix_unary_operation_syntax(
        &self,
        p: PostfixUnaryOperationSyntax,
    ) -> TypedPostfixUnaryOp {
        let PostfixUnaryOperationSyntax { target, operator } = p;
        let target = self.expr(*target);
        TypedPostfixUnaryOp {
            target: Box::new(target),
            kind: operator.token,
            type_: None,
        }
    }

    pub fn array_syntax(&self, a: ArraySyntax) -> TypedArray {
        TypedArray {
            elements: a.values.into_iter().map(|e| self.expr(e.element)).collect(),
            type_: None,
        }
    }

    pub fn subscript_syntax(&self, s: SubscriptSyntax) -> TypedSubscript {
        let target = Box::new(self.expr(*s.target));
        let indexes: Vec<TypedExpr> = s.idx_or_keys.into_iter().map(|i| self.expr(i)).collect();
        TypedSubscript {
            target,
            indexes,
            type_: None,
        }
    }

    pub fn member_syntax(&self, m: MemberSyntax) -> TypedInstanceMember {
        let MemberSyntax {
            target,
            name,
            navigation_operator,
        } = m;
        let target = self.expr(*target);
        TypedInstanceMember {
            target: Box::new(target),
            name: name.token,
            is_safe: navigation_operator.token.ends_with("?"),
            type_: None,
        }
    }

    pub fn call_syntax(&self, c: CallExprSyntax) -> TypedCall {
        let CallExprSyntax {
            target,
            args,
            tailing_lambda,
        } = c;
        let mut args: Vec<TypedCallArg> = args
            .into_iter()
            .map(|a| TypedCallArg {
                label: a.label,
                arg: Box::new(self.expr(*a.arg)),
                is_vararg: a.is_vararg,
            })
            .collect();
        if let Some(lambda) = tailing_lambda {
            args.insert(
                args.len(),
                TypedCallArg {
                    label: None,
                    arg: Box::new(TypedExpr::Lambda(self.lambda_syntax(lambda))),
                    is_vararg: false,
                },
            )
        }
        TypedCall {
            target: Box::new(self.expr(*target)),
            args: args,
            type_: None,
        }
    }

    pub fn if_syntax(&self, i: IfExprSyntax) -> TypedIf {
        let IfExprSyntax {
            condition,
            body,
            else_body,
        } = i;
        let block = self.block(body);
        let type_ = if else_body == None {
            TypedType::noting()
        } else {
            block.type_().unwrap_or(TypedType::noting())
        };
        TypedIf {
            condition: Box::new(self.expr(*condition)),
            body: block,
            else_body: else_body.map(|b| self.block(b)),
            type_: Some(type_),
        }
    }

    pub fn lambda_syntax(&self, l: LambdaSyntax) -> TypedLambda {
        let LambdaSyntax { stmts: _ } = l;
        todo!();
        TypedLambda {
            args: vec![],
            body: TypedBlock { body: vec![] },
        }
    }

    pub fn return_syntax(&self, r: ReturnSyntax) -> TypedReturn {
        let value = r.value.map(|v| Box::new(self.expr(*v)));
        TypedReturn { value }
    }

    pub fn type_cast(&self, t: TypeCastSyntax) -> TypedTypeCast {
        TypedTypeCast {
            target: Box::new(self.expr(*t.target)),
            is_safe: t.operator.ends_with("?"),
            type_: Some(self.type_(t.type_)),
        }
    }

    pub fn block(&self, block: BlockSyntax) -> TypedBlock {
        TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }
}
