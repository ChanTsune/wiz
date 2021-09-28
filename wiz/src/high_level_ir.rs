use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDecl, TypedFun, TypedFunBody, TypedInitializer,
    TypedMemberFunction, TypedStoredProperty, TypedStruct, TypedValueArgDef, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember, TypedLiteral,
    TypedName, TypedReturn, TypedSubscript, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedType, TypedTypeParam, TypedValueType};
use crate::syntax::block::Block;
use crate::syntax::decl::{
    Decl, FunSyntax, InitializerSyntax, MethodSyntax, StoredPropertySyntax, StructPropertySyntax,
    StructSyntax, VarSyntax,
};
use crate::syntax::expr::{CallExprSyntax, Expr, NameExprSyntax, ReturnSyntax, SubscriptSyntax};
use crate::syntax::file::{FileSyntax, WizFile};
use crate::syntax::fun::arg_def::ArgDef;
use crate::syntax::fun::body_def::FunBody;
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::stmt::{AssignmentStmt, LoopStmt, Stmt};
use crate::syntax::type_name::{TypeName, TypeParam};
use crate::utils::path_string_to_page_name;
use std::option::Option::Some;

pub mod type_resolver;
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

    pub fn file(&mut self, f: WizFile) -> TypedFile {
        TypedFile {
            name: path_string_to_page_name(f.name),
            body: self.file_syntax(f.syntax),
        }
    }

    pub fn file_syntax(&mut self, f: FileSyntax) -> Vec<TypedDecl> {
        f.body.into_iter().map(|d| self.decl(d)).collect()
    }

    pub fn stmt(&mut self, s: Stmt) -> TypedStmt {
        match s {
            Stmt::Decl { decl } => TypedStmt::Decl(self.decl(decl)),
            Stmt::Expr { expr } => TypedStmt::Expr(self.expr(expr)),
            Stmt::Assignment(a) => TypedStmt::Assignment(self.assignment(a)),
            Stmt::Loop(l) => TypedStmt::Loop(self.loop_stmt(l)),
        }
    }

    pub fn assignment(&mut self, a: AssignmentStmt) -> TypedAssignmentStmt {
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

    pub fn loop_stmt(&mut self, l: LoopStmt) -> TypedLoopStmt {
        match l {
            LoopStmt::While { condition, block } => TypedLoopStmt::While(TypedWhileLoopStmt {
                condition: self.expr(condition),
                block: self.block(block),
            }),
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

    pub fn decl(&mut self, d: Decl) -> TypedDecl {
        match d {
            Decl::Var(v) => TypedDecl::Var(self.var_syntax(v)),
            Decl::Fun(f) => TypedDecl::Fun(self.fun_syntax(f)),
            Decl::Struct(s) => {
                let struct_ = self.struct_syntax(s);
                let struct_ = self.default_init_if_needed(struct_);
                TypedDecl::Struct(struct_)
            }
            Decl::Class { .. } => TypedDecl::Class,
            Decl::Enum { .. } => TypedDecl::Enum,
            Decl::Protocol { .. } => TypedDecl::Protocol,
            Decl::Extension { .. } => TypedDecl::Extension,
            Decl::Use(_) => TypedDecl::Use,
        }
    }

    pub fn var_syntax(&mut self, v: VarSyntax) -> TypedVar {
        let expr = self.expr(v.value);
        TypedVar {
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
            ArgDef::Self_ => TypedArgDef::Self_(None),
        }
    }

    pub fn fun_body(&mut self, body: FunBody) -> TypedFunBody {
        match body {
            FunBody::Block { block } => TypedFunBody::Block(self.block(block)),
            FunBody::Expr { expr } => TypedFunBody::Expr(self.expr(expr)),
        }
    }

    pub fn fun_syntax(&mut self, f: FunSyntax) -> TypedFun {
        let args: Vec<TypedArgDef> = f.arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        let body = match f.body {
            None => None,
            Some(b) => Some(self.fun_body(b)),
        };

        TypedFun {
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
        TypedType::Value(TypedValueType {
            package: Some(Package::global()),
            name: tn.name,
            type_args: tn
                .type_args
                .map(|v| v.into_iter().map(|t| self.type_(t)).collect()),
        })
    }

    fn type_param(&self, tp: TypeParam) -> TypedTypeParam {
        TypedTypeParam {
            name: tp.name,
            type_constraint: tp.type_constraints.map_or(vec![], |v| vec![self.type_(v)]),
        }
    }

    pub fn struct_syntax(&mut self, s: StructSyntax) -> TypedStruct {
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
            };
        }
        TypedStruct {
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

    pub fn initializer_syntax(&mut self, init: InitializerSyntax) -> TypedInitializer {
        TypedInitializer {
            args: init.args.into_iter().map(|a| self.arg_def(a)).collect(),
            body: self.fun_body(init.body),
        }
    }

    pub fn member_function(&mut self, member_function: MethodSyntax) -> TypedMemberFunction {
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

    pub fn expr(&mut self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name(n) => TypedExpr::Name(self.name_syntax(n)),
            Expr::Literal(literal) => match literal {
                LiteralSyntax::Integer(value) => TypedExpr::Literal(TypedLiteral::Integer {
                    value: value.token,
                    type_: Some(TypedType::int64()),
                }),
                LiteralSyntax::FloatingPoint(value) => {
                    TypedExpr::Literal(TypedLiteral::FloatingPoint {
                        value: value.token,
                        type_: Some(TypedType::double()),
                    })
                }
                LiteralSyntax::String {
                    open_quote: _,
                    value,
                    close_quote: _,
                } => TypedExpr::Literal(TypedLiteral::String {
                    value,
                    type_: Some(TypedType::string()),
                }),
                LiteralSyntax::Boolean(syntax) => TypedExpr::Literal(TypedLiteral::Boolean {
                    value: syntax.token,
                    type_: Some(TypedType::bool()),
                }),
                LiteralSyntax::Null => {
                    TypedExpr::Literal(TypedLiteral::NullLiteral { type_: None })
                }
            },
            Expr::BinOp { left, kind, right } => {
                let left = Box::new(self.expr(*left));
                let right = Box::new(self.expr(*right));
                TypedExpr::BinOp(TypedBinOp {
                    left: left,
                    kind: kind,
                    right: right,
                    type_: None,
                })
            }
            Expr::UnaryOp {
                target,
                prefix,
                kind,
            } => {
                let target = self.expr(*target);
                TypedExpr::UnaryOp(TypedUnaryOp {
                    target: Box::new(target),
                    prefix: prefix,
                    kind: kind,
                    type_: None,
                })
            }
            Expr::Subscript(s) => TypedExpr::Subscript(self.subscript_syntax(s)),
            Expr::Member {
                target,
                name,
                is_safe,
            } => {
                let target = self.expr(*target);
                TypedExpr::Member(TypedInstanceMember {
                    target: Box::new(target),
                    name,
                    is_safe,
                    type_: None,
                })
            }
            Expr::List { .. } => TypedExpr::List,
            Expr::Tuple { .. } => TypedExpr::Tuple,
            Expr::Dict { .. } => TypedExpr::Dict,
            Expr::StringBuilder { .. } => TypedExpr::StringBuilder,
            Expr::Call(c) => TypedExpr::Call(self.call_syntax(c)),
            Expr::If {
                condition,
                body,
                else_body,
            } => {
                let block = self.block(body);
                let type_ = if else_body == None {
                    TypedType::noting()
                } else {
                    block.type_().unwrap_or(TypedType::noting())
                };
                TypedExpr::If(TypedIf {
                    condition: Box::new(self.expr(*condition)),
                    body: block,
                    else_body: else_body.map(|b| self.block(b)),
                    type_: Some(type_),
                })
            }
            Expr::When { .. } => TypedExpr::When,
            Expr::Lambda { .. } => TypedExpr::Lambda,
            Expr::Return(r) => TypedExpr::Return(self.return_syntax(r)),
            Expr::TypeCast { .. } => TypedExpr::TypeCast,
        }
    }

    pub fn name_syntax(&self, n: NameExprSyntax) -> TypedName {
        let NameExprSyntax { name } = n;
        TypedName {
            package: None,
            name,
            type_: None,
        }
    }

    pub fn subscript_syntax(&mut self, s: SubscriptSyntax) -> TypedSubscript {
        let target = Box::new(self.expr(*s.target));
        let indexes: Vec<TypedExpr> = s.idx_or_keys.into_iter().map(|i| self.expr(i)).collect();
        TypedSubscript {
            target,
            indexes,
            type_: None,
        }
    }

    pub fn call_syntax(&mut self, c: CallExprSyntax) -> TypedCall {
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
                    arg: Box::new(TypedExpr::Lambda /* TODO: use lambda */),
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

    pub fn return_syntax(&mut self, r: ReturnSyntax) -> TypedReturn {
        let value = r.value.map(|v| Box::new(self.expr(*v)));
        let t = match &value {
            Some(v) => v.type_(),
            None => None,
        };
        TypedReturn { value, type_: t }
    }

    pub fn block(&mut self, block: Block) -> TypedBlock {
        TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }
}
