use crate::high_level_ir::typed_decl::{TypedArgDef, TypedDecl, TypedFun, TypedFunBody};
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{TypedBlock, TypedStmt};
use crate::high_level_ir::typed_type::TypedType;
use crate::middle_level_ir::ml_decl::{MLArgDef, MLDecl, MLFunBody};
use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLBlock, MLStmt};
use crate::middle_level_ir::ml_type::MLType;
use std::process::exit;

pub mod ml_decl;
pub mod ml_expr;
pub mod ml_file;
pub mod ml_stmt;
pub mod ml_type;

pub struct HLIR2MLIR {}

impl HLIR2MLIR {
    pub fn new() -> Self {
        HLIR2MLIR {}
    }

    pub fn type_(&self, t: TypedType) -> MLType {
        let mut pkg = t.package.names;
        pkg.append(&mut vec![t.name]);
        MLType {
            name: pkg.join("::"),
        }
    }

    pub fn file(&self, f: TypedFile) -> MLFile {
        MLFile {
            body: f.body.into_iter().map(|d| self.decl(d)).collect(),
        }
    }

    pub fn stmt(&self, s: TypedStmt) -> MLStmt {
        match s {
            TypedStmt::Expr(e) => MLStmt::Expr(self.expr(e)),
            TypedStmt::Decl(d) => MLStmt::Decl(self.decl(d)),
            TypedStmt::Assignment => MLStmt::Assignment,
            TypedStmt::Loop => MLStmt::Loop,
        }
    }

    pub fn decl(&self, d: TypedDecl) -> MLDecl {
        match d {
            TypedDecl::Var {
                is_mut,
                name,
                type_,
                value,
            } => {
                let expr = self.expr(value);
                MLDecl::Var {
                    is_mute: is_mut,
                    name: name,
                    type_: self.type_(type_.unwrap()),
                    value: expr,
                }
            }
            TypedDecl::Fun(TypedFun {
                modifiers,
                name,
                arg_defs,
                body,
                return_type,
            }) => MLDecl::Fun {
                modifiers,
                name,
                arg_defs: arg_defs.into_iter().map(|a| self.arg_def(a)).collect(),
                return_type: self.type_(return_type),
                body: body.map(|b| self.fun_body(b)),
            },
            TypedDecl::Struct => exit(-1),
            TypedDecl::Class => exit(-1),
            TypedDecl::Enum => exit(-1),
            TypedDecl::Protocol => exit(-1),
            TypedDecl::Extension => exit(-1),
        }
    }

    pub fn expr(&self, e: TypedExpr) -> MLExpr {
        match e {
            TypedExpr::Name { .. } => MLExpr::Name,
            TypedExpr::Literal(_) => MLExpr::Literal,
            TypedExpr::BinOp { .. } => MLExpr::Call,
            TypedExpr::UnaryOp { .. } => exit(-1),
            TypedExpr::Subscript => exit(-1),
            TypedExpr::List => exit(-1),
            TypedExpr::Tuple => exit(-1),
            TypedExpr::Dict => exit(-1),
            TypedExpr::StringBuilder => exit(-1),
            TypedExpr::Call { .. } => MLExpr::Call,
            TypedExpr::If => MLExpr::If,
            TypedExpr::When => exit(-1),
            TypedExpr::Lambda => exit(-1),
            TypedExpr::Return => exit(-1),
            TypedExpr::TypeCast => exit(-1),
        }
    }

    pub fn arg_def(&self, e: TypedArgDef) -> MLArgDef {
        MLArgDef {
            name: e.name,
            type_: self.type_(e.type_),
        }
    }

    pub fn fun_body(&self, b: TypedFunBody) -> MLFunBody {
        match b {
            TypedFunBody::Expr(e) => MLFunBody {
                body: vec![MLStmt::Expr(self.expr(e))],
            },
            TypedFunBody::Block(b) => MLFunBody {
                body: self.block(b).body,
            },
        }
    }

    pub fn block(&self, b: TypedBlock) -> MLBlock {
        MLBlock {
            body: b.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }
}
