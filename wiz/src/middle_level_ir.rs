use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_type::TypedType;
use crate::middle_level_ir::ml_decl::MLDecl;
use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_type::MLType;
use std::process::exit;

pub mod ml_decl;
pub mod ml_expr;
pub mod ml_file;
pub mod ml_type;

pub struct HLIR2MLIR {}

impl HLIR2MLIR {
    fn new() -> Self {
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
        MLFile { body: vec![] }
    }

    pub fn decl(&self, d: TypedDecl) -> MLDecl {
        match d {
            TypedDecl::Var {
                is_mut,
                name,
                type_,
                value,
            } => MLDecl::Var {
                is_mute: is_mut,
                name: name,
                type_: self.type_(type_),
                value: MLExpr::Name,
            },
            TypedDecl::Fun {
                modifiers,
                name,
                arg_defs,
                body,
                return_type,
            } => MLDecl::Fun {
                modifiers,
                name,
                arg_defs: vec![],
                return_type: self.type_(return_type),
                body: None,
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
            TypedExpr::Literal(_) => exit(-1),
            TypedExpr::BinOp { .. } => exit(-1),
            TypedExpr::UnaryOp { .. } => exit(-1),
            TypedExpr::Subscript => exit(-1),
            TypedExpr::List => exit(-1),
            TypedExpr::Tuple => exit(-1),
            TypedExpr::Dict => exit(-1),
            TypedExpr::StringBuilder => exit(-1),
            TypedExpr::Call { .. } => exit(-1),
            TypedExpr::If => exit(-1),
            TypedExpr::When => exit(-1),
            TypedExpr::Lambda => exit(-1),
            TypedExpr::Return => exit(-1),
            TypedExpr::TypeCast => exit(-1),
        }
    }
}
