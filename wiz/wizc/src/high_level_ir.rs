use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDecl, TypedFun, TypedFunBody, TypedInitializer,
    TypedMemberFunction, TypedStoredProperty, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedArray, TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedIf,
    TypedInstanceMember, TypedLambda, TypedLiteral, TypedName, TypedPostfixUnaryOp,
    TypedPostfixUnaryOperator, TypedPrefixUnaryOp, TypedPrefixUnaryOperator, TypedReturn,
    TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentAndOperator, TypedAssignmentStmt,
    TypedBlock, TypedForStmt, TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{
    Package, TypedNamedValueType, TypedPackage, TypedType, TypedTypeParam, TypedValueType,
};
use crate::high_level_ir::typed_use::TypedUse;
use crate::utils::path_string_to_page_name;
use std::option::Option::Some;
use wiz_syntax::syntax::annotation::AnnotationsSyntax;
use wiz_syntax::syntax::block::BlockSyntax;
use wiz_syntax::syntax::declaration::fun_syntax::{ArgDef, FunBody, FunSyntax};
use wiz_syntax::syntax::declaration::VarSyntax;
use wiz_syntax::syntax::declaration::{
    Decl, InitializerSyntax, StoredPropertySyntax, StructPropertySyntax, StructSyntax, UseSyntax,
};
use wiz_syntax::syntax::expression::{
    ArraySyntax, BinaryOperationSyntax, CallExprSyntax, Expr, IfExprSyntax, LambdaSyntax,
    MemberSyntax, NameExprSyntax, PostfixUnaryOperationSyntax, PrefixUnaryOperationSyntax,
    ReturnSyntax, SubscriptSyntax, TypeCastSyntax, UnaryOperationSyntax,
};
use wiz_syntax::syntax::file::{FileSyntax, SourceSet, WizFile};
use wiz_syntax::syntax::literal::LiteralSyntax;
use wiz_syntax::syntax::statement::{
    AssignmentStmt, ForLoopSyntax, LoopStmt, Stmt, WhileLoopSyntax,
};
use wiz_syntax::syntax::type_name::{TypeName, TypeParam, UserTypeName};

pub mod type_resolver;
pub mod typed_annotation;
pub mod typed_decl;
pub mod typed_expr;
pub mod typed_file;
pub mod typed_stmt;
pub mod typed_type;
pub mod typed_use;

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
        let WizFile { name, syntax } = f;
        let mut uses = vec![];
        let mut others = vec![];
        for l in syntax.body.into_iter() {
            if let Decl::Use(u) = l {
                uses.push(self.use_syntax(u));
            } else {
                others.push(l);
            }
        }

        TypedFile {
            name: path_string_to_page_name(name),
            uses,
            body: self.file_syntax(FileSyntax { body: others }),
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
                    .map(|a| a.element.token)
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
                    operator: match &*a.operator.token {
                        "+=" => TypedAssignmentAndOperator::Add,
                        "-=" => TypedAssignmentAndOperator::Sub,
                        "*=" => TypedAssignmentAndOperator::Mul,
                        "/=" => TypedAssignmentAndOperator::Div,
                        "%=" => TypedAssignmentAndOperator::Mod,
                        o => panic!("unknown operator {:?}", o),
                    },
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
            LoopStmt::For(ForLoopSyntax {
                for_keyword: _,
                values,
                in_keyword: _,
                iterator,
                block,
            }) => TypedLoopStmt::For(TypedForStmt {
                values: values.into_iter().map(|i| i.token).collect(),
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
            Decl::Use(_) => {
                panic!("Never execution branch executed!!")
            }
        }
    }

    pub fn var_syntax(&self, v: VarSyntax) -> TypedVar {
        let expr = self.expr(v.value);
        TypedVar {
            annotations: self.annotations(v.annotations),
            package: TypedPackage::Raw(Package::new()),
            is_mut: v.mutability_keyword.token == "var",
            name: v.name.token,
            type_: v.type_.map(|t| self.type_(t)),
            value: expr,
        }
    }

    pub fn arg_def(&self, a: ArgDef) -> TypedArgDef {
        match a {
            ArgDef::Value(a) => TypedArgDef {
                label: match a.label {
                    None => a.name.token.clone(),
                    Some(label) => label.token,
                },
                name: a.name.token,
                type_: self.type_(a.type_name),
            },
            ArgDef::Self_(s) => match s.reference {
                None => TypedArgDef {
                    label: "_".to_string(),
                    name: "self".to_string(),
                    type_: TypedType::Self_,
                },
                Some(_) => TypedArgDef {
                    label: "_".to_string(),
                    name: "self".to_string(),
                    type_: TypedType::Self_, // TODO: Reference
                },
            },
        }
    }

    pub fn fun_body(&self, body: FunBody) -> TypedFunBody {
        match body {
            FunBody::Block(block) => TypedFunBody::Block(self.block(block)),
            FunBody::Expr(expr) => TypedFunBody::Expr(self.expr(expr)),
        }
    }

    pub fn fun_syntax(&self, f: FunSyntax) -> TypedFun {
        let args: Vec<TypedArgDef> = f
            .arg_defs
            .elements
            .into_iter()
            .map(|a| self.arg_def(a.element))
            .collect();
        let body = f.body.map(|b| self.fun_body(b));

        TypedFun {
            annotations: self.annotations(f.annotations),
            package: TypedPackage::Raw(Package::new()),
            modifiers: f.modifiers.modifiers.into_iter().map(|m| m.token).collect(),
            name: f.name.token,
            type_params: f.type_params.map(|v| {
                v.elements
                    .into_iter()
                    .map(|p| TypedTypeParam {
                        name: p.element.name.token,
                        type_constraint: match p.element.type_constraint {
                            None => {
                                vec![]
                            }
                            Some(tn) => {
                                vec![self.type_(tn.constraint)]
                            }
                        },
                    })
                    .collect()
            }),
            arg_defs: args,
            body,
            return_type: f.return_type.map(|t| self.type_(t)),
        }
    }

    pub fn type_(&self, tn: TypeName) -> TypedType {
        match tn {
            TypeName::Simple(stn) => TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                package: TypedPackage::Raw(Package::new()),
                name: stn.name.token,
                type_args: stn.type_args.map(|v| {
                    v.elements
                        .into_iter()
                        .map(|t| self.type_(t.element))
                        .collect()
                }),
            })),
            TypeName::Decorated(d) => {
                let t = self.type_(d.type_);
                match &*d.decoration.token {
                    "&" => TypedType::Value(TypedValueType::Reference(Box::new(t))),
                    "*" => TypedType::Value(TypedValueType::Pointer(Box::new(t))),
                    a => panic!("Unexpected token {}", a),
                }
            }
            TypeName::NameSpaced(n) => {
                let UserTypeName {
                    name_space,
                    type_name,
                } = *n;
                TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                    package: TypedPackage::Raw(Package::from(
                        name_space
                            .into_iter()
                            .map(|i| i.simple_type.name.token)
                            .collect::<Vec<String>>(),
                    )),
                    name: type_name.name.token,
                    type_args: type_name.type_args.map(|v| {
                        v.elements
                            .into_iter()
                            .map(|t| self.type_(t.element))
                            .collect()
                    }),
                }))
            }
        }
    }

    fn type_param(&self, tp: TypeParam) -> TypedTypeParam {
        TypedTypeParam {
            name: tp.name.token,
            type_constraint: tp
                .type_constraint
                .map_or(vec![], |v| vec![self.type_(v.constraint)]),
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
            package: TypedPackage::Raw(Package::new()),
            name: s.name.token,
            type_params: s.type_params.map(|v| {
                v.elements
                    .into_iter()
                    .map(|tp| self.type_param(tp.element))
                    .collect()
            }),
            initializers,
            stored_properties,
            computed_properties,
            member_functions,
        }
    }

    fn default_init_if_needed(&self, mut s: TypedStruct) -> TypedStruct {
        let args: Vec<TypedArgDef> = s
            .stored_properties
            .iter()
            .map(|p| TypedArgDef {
                label: p.name.clone(),
                name: p.name.clone(),
                type_: p.type_.clone(),
            })
            .collect();
        if s.initializers.is_empty() {
            let struct_type = TypedNamedValueType {
                package: TypedPackage::Raw(Package::new()),
                name: s.name.clone(),
                type_args: None,
            };
            s.initializers.push(TypedInitializer {
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
                                            package: TypedPackage::Raw(Package::new()),
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedValueType::Value(
                                                struct_type.clone(),
                                            ))),
                                        })),
                                        name: p.name.clone(),
                                        is_safe: false,
                                        type_: Some(p.type_.clone()),
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: TypedPackage::Raw(Package::new()),
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
            name: p.name.token,
            type_: self.type_(p.type_),
        }
    }

    pub fn initializer_syntax(&self, init: InitializerSyntax) -> TypedInitializer {
        TypedInitializer {
            args: init
                .args
                .elements
                .into_iter()
                .map(|a| self.arg_def(a.element))
                .collect(),
            body: self.fun_body(init.body),
        }
    }

    pub fn member_function(&self, member_function: FunSyntax) -> TypedMemberFunction {
        let FunSyntax {
            annotations: _,
            modifiers: _,
            fun_keyword: _,
            name,
            type_params,
            arg_defs,
            return_type,
            type_constraints,
            body,
        } = member_function;

        let rt = return_type.map(|r| self.type_(r));
        let fb = body.map(|b| self.fun_body(b));
        TypedMemberFunction {
            name: name.token,
            arg_defs: arg_defs
                .elements
                .into_iter()
                .map(|a| self.arg_def(a.element))
                .collect(),
            type_params: type_params.map(|tps| {
                tps.elements
                    .into_iter()
                    .map(|p| self.type_param(p.element))
                    .collect()
            }),
            body: fb,
            return_type: rt,
        }
    }

    pub fn use_syntax(&self, u: UseSyntax) -> TypedUse {
        let mut names: Vec<_> = u
            .package_name
            .names
            .into_iter()
            .map(|i| i.name.token)
            .collect();
        names.push(u.used_name.token);
        TypedUse {
            annotations: self.annotations(u.annotations),
            package: Package { names },
            alias: u.alias.map(|a| a.name.token),
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
                type_: None,
            },
            LiteralSyntax::FloatingPoint(value) => TypedLiteral::FloatingPoint {
                value: value.token,
                type_: None,
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
        let name_space = name_space
            .elements
            .into_iter()
            .map(|e| e.name.token)
            .collect::<Vec<String>>();
        TypedName {
            package: if name_space.is_empty() {
                TypedPackage::Raw(Package::new())
            } else {
                TypedPackage::Raw(Package::from(name_space))
            },
            name: name.token,
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
            operator: match &*kind.token {
                "+" => TypedBinaryOperator::Add,
                "-" => TypedBinaryOperator::Sub,
                "*" => TypedBinaryOperator::Mul,
                "/" => TypedBinaryOperator::Div,
                "%" => TypedBinaryOperator::Mod,
                "==" => TypedBinaryOperator::Equal,
                ">=" => TypedBinaryOperator::GrateThanEqual,
                ">" => TypedBinaryOperator::GrateThan,
                "<=" => TypedBinaryOperator::LessThanEqual,
                "<" => TypedBinaryOperator::LessThan,
                "!=" => TypedBinaryOperator::NotEqual,
                _ => TypedBinaryOperator::InfixFunctionCall(kind.token),
            },
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
            operator: match &*operator.token {
                "+" => TypedPrefixUnaryOperator::Positive,
                "-" => TypedPrefixUnaryOperator::Negative,
                "*" => TypedPrefixUnaryOperator::Dereference,
                "&" => TypedPrefixUnaryOperator::Reference,
                "!" => TypedPrefixUnaryOperator::Not,
                _ => panic!(),
            },
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
            operator: match &*operator.token {
                "!!" => TypedPostfixUnaryOperator::Unwrap,
                _ => panic!(),
            },
            type_: None,
        }
    }

    pub fn array_syntax(&self, a: ArraySyntax) -> TypedArray {
        TypedArray {
            elements: a
                .elements
                .into_iter()
                .map(|e| self.expr(e.element))
                .collect(),
            type_: None,
        }
    }

    pub fn subscript_syntax(&self, s: SubscriptSyntax) -> TypedSubscript {
        let target = Box::new(self.expr(*s.target));
        let indexes: Vec<TypedExpr> = s
            .idx_or_keys
            .elements
            .into_iter()
            .map(|i| self.expr(i.element))
            .collect();
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
            is_safe: navigation_operator.token.ends_with('?'),
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
            .unwrap_or_default()
            .elements
            .into_iter()
            .map(|a| TypedCallArg {
                label: a.element.label.map(|l| l.token),
                arg: Box::new(self.expr(*a.element.arg)),
                is_vararg: a.element.asterisk.is_some(),
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
            args,
            type_: None,
        }
    }

    pub fn if_syntax(&self, i: IfExprSyntax) -> TypedIf {
        let IfExprSyntax {
            if_keyword: _,
            condition,
            body,
            else_body,
        } = i;
        let block = self.block(body);
        TypedIf {
            condition: Box::new(self.expr(*condition)),
            body: block,
            else_body: else_body.map(|b| self.block(b.body)),
            type_: None,
        }
    }

    pub fn lambda_syntax(&self, l: LambdaSyntax) -> TypedLambda {
        todo!("{:?}", l);
        let LambdaSyntax {
            open: _,
            stmts: _,
            close: _,
        } = l;
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
            is_safe: t.operator.token.ends_with('?'),
            type_: Some(self.type_(t.type_)),
        }
    }

    pub fn block(&self, block: BlockSyntax) -> TypedBlock {
        TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }
}
