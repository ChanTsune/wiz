use crate::ast::file::File;
use crate::high_level_ir::typed_file::TypedFile;
use crate::ast::decl::Decl;
use crate::high_level_ir::typed_decl::{TypedDecl, TypedArgDef, TypedFunBody};
use crate::high_level_ir::typed_expr::{TypedExpr, TypedLiteral, TypedCallArg};
use crate::parser::nom::declaration::decl;
use crate::ast::expr::Expr;
use crate::ast::literal::Literal;
use crate::high_level_ir::typed_type::{TypedType, Package};
use std::collections::HashMap;
use crate::ast::type_name::TypeName;
use crate::ast::fun::body_def::FunBody;
use crate::high_level_ir::typed_stmt::{TypedBlock, TypedStmt};
use crate::ast::stmt::Stmt;
use std::option::Option::Some;

mod typed_decl;
mod typed_expr;
mod typed_type;
mod typed_file;
mod typed_stmt;

pub struct AstToHLIR {
    environment: Vec<HashMap<String, TypedType>>
}


impl AstToHLIR {

    fn new() -> Self {
        AstToHLIR {
            environment: vec![]
        }
    }

    fn push_env(&mut self) {
        self.environment.push(HashMap::new())
    }

    fn pop_env(&mut self) {
        self.environment.pop();
    }

    fn get_type_by(&self, name: String) -> Option<TypedType> {
        for env in self.environment.iter().rev() {
            if let Some(v) = env.get(&*name) {
                return Some(v.clone())
            }
        }
        None
    }

    fn resolve_by_type_name(&self, type_name: TypeName) -> Option<TypedType> {
        None
    }

    fn resolve_by_binop(&self, left_type: &TypedType, kind: &String, right_type: &TypedType) -> Option<TypedType> {
        None
    }

    fn resolve_by_unaryop(&self, target_type: &TypedType, kind: &String) -> Option<TypedType> {
        None
    }

    pub fn file(&self, f: File) -> TypedFile {
        TypedFile {
            body: f.body.into_iter().map(|d|{self.decl(d)}).collect()
        }
    }

    pub fn stmt(&self, s: Stmt) -> TypedStmt {
        match s {
            Stmt::Decl { decl } => {
                TypedStmt::Decl(self.decl(decl))
            }
            Stmt::Expr { expr } => {
                TypedStmt::Expr(self.expr(expr))
            }
        }
    }

    pub fn decl(&self, d: Decl) -> TypedDecl {
        match d {
            Decl::Var { is_mut, name, type_, value } => {
                let expr = self.expr(value);
                TypedDecl::Var {
                    is_mut: is_mut,
                    name: name,
                    type_: match type_ {
                        Some(t) => {
                            self.resolve_by_type_name(t).unwrap()
                        }
                        None => {
                            expr.type_()
                        }
                    },
                    value: TypedExpr::Subscript
                }
            }
            Decl::Fun { modifiers, name, arg_defs, return_type, body } => {
                TypedDecl::Fun {
                    modifiers: modifiers,
                    name: name,
                    arg_defs: arg_defs.into_iter().map(|a|{ TypedArgDef{
                        label: a.label,
                        name: a.name,
                        type_: self.resolve_by_type_name(a.type_name).unwrap()
                    }}).collect(),
                    body: body.map(|b|{
                        match b {
                            FunBody::Block { block } => {
                                TypedFunBody::Block(TypedBlock {
                                    body: block.body.into_iter().map(|s|{
                                        self.stmt(s)
                                    }).collect()
                                })
                            }
                            FunBody::Expr { expr } => {
                                TypedFunBody::Expr(self.expr(expr))
                            }
                        }
                    }),
                    return_type: self.resolve_by_type_name(return_type).unwrap()
                }
            }
            Decl::Struct { .. } => {
                TypedDecl::Struct
            }
            Decl::Class { .. } => {
                TypedDecl::Class
            }
            Decl::Enum { .. } => {
                TypedDecl::Enum
            }
            Decl::Protocol { .. } => {
                TypedDecl::Protocol
            }
            Decl::Extension { .. } => {
                TypedDecl::Extension
            }
        }
    }

    pub fn expr(&self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name { name } => {
                TypedExpr::Name {
                    name: name.clone(),
                    type_: self.get_type_by(name).unwrap()
                }
            }
            Expr::Literal { literal } => {
                match literal {
                    Literal::IntegerLiteral { value } => {
                        TypedExpr::Literal(TypedLiteral::Integer { value, type_: TypedType{ package: Package{ names: vec![] }, name: "Int64".to_string() } })
                    }
                    Literal::FloatingPointLiteral { value } => {
                        TypedExpr::Literal(TypedLiteral::FloatingPoint { value, type_: TypedType { package: Package { names: vec![] }, name: "Double".to_string() } })
                    }
                    Literal::StringLiteral { value } => {
                        TypedExpr::Literal(TypedLiteral::String { value, type_: TypedType { package: Package { names: vec![] }, name: "String".to_string() } })
                    }
                    Literal::BooleanLiteral { value } => {
                        TypedExpr::Literal(TypedLiteral::Boolean { value, type_: TypedType { package: Package { names: vec![] }, name: "Bool".to_string() } })
                    }
                    Literal::NullLiteral => {
                        TypedExpr::Literal(TypedLiteral::NullLiteral { type_: TypedType { package: Package { names: vec![] }, name: "Option<*>".to_string() } })
                    }
                }
            }
            Expr::BinOp { left, kind, right } => {
                let left = Box::new(self.expr(*left));
                let right = Box::new(self.expr(*right));
                let type_ = self.resolve_by_binop(&left.type_(), &kind,&right.type_());
                TypedExpr::BinOp {
                    left: left,
                    kind: kind,
                    right: right,
                    type_: type_.unwrap()
                }
            }
            Expr::UnaryOp { target, prefix, kind } => {
                let target = self.expr(*target);
                let type_ = self.resolve_by_unaryop(&target.type_(), &kind);
                TypedExpr::UnaryOp {
                    target: Box::new(target),
                    prefix: prefix,
                    kind: kind,
                    type_: type_.unwrap()
                }
            }
            Expr::Subscript { .. } => {
                TypedExpr::Subscript
            }
            Expr::List { .. } => {
                TypedExpr::List
            }
            Expr::Tuple { .. } => {
                TypedExpr::Tuple
            }
            Expr::Dict { .. } => {
                TypedExpr::Dict
            }
            Expr::StringBuilder { .. } => {
                TypedExpr::StringBuilder
            }
            Expr::Call { target, args, tailing_lambda } => {
                let mut args:Vec<TypedCallArg> = args.into_iter().map(|a|{
                    TypedCallArg {
                        label: a.label,
                        arg: Box::new(self.expr(*a.arg)),
                        is_vararg: a.is_vararg
                    }
                }).collect();
                if let Some(lambda) = tailing_lambda{
                    args.insert(args.len(), TypedCallArg {
                        label: None,
                        arg: Box::new(TypedExpr::Lambda /* TODO: use lambda */),
                        is_vararg: false
                    })
                }
                // TODO: resolve call type
                TypedExpr::Call { target: Box::new(self.expr(*target)), args: args, type_: TypedType::noting() }
            }
            Expr::If { .. } => {
                TypedExpr::If
            }
            Expr::When { .. } => {
                TypedExpr::When
            }
            Expr::Lambda { .. } => {
                TypedExpr::Lambda
            }
            Expr::Return { .. } => {
                TypedExpr::Return
            }
            Expr::TypeCast { .. } => {
                TypedExpr::TypeCast
            }
        }
    }
}

