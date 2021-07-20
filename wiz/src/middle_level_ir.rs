use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{TypedExpr, TypedIf, TypedLiteral, TypedName, TypedReturn};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt};
use crate::high_level_ir::typed_type::TypedType;
use crate::middle_level_ir::ml_decl::{
    MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar,
};
use crate::middle_level_ir::ml_expr::{
    MLBinOp, MLBinopKind, MLCall, MLCallArg, MLExpr, MLIf, MLLiteral, MLName, MLReturn,
};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLAssignmentStmt, MLBlock, MLLoopStmt, MLStmt};
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
            TypedStmt::Assignment(a) => MLStmt::Assignment(self.assignment(a)),
            TypedStmt::Loop(l) => MLStmt::Loop(self.loop_stmt(l)),
        }
    }

    pub fn assignment(&self, a: TypedAssignmentStmt) -> MLAssignmentStmt {
        MLAssignmentStmt {
            target: a.target,
            value: self.expr(a.value),
        }
    }

    pub fn loop_stmt(&self, l: TypedLoopStmt) -> MLLoopStmt {
        match l {
            TypedLoopStmt::While(w) => MLLoopStmt {
                condition: self.expr(w.condition),
                block: self.block(w.block),
            },
            TypedLoopStmt::For(_) => exit(-1),
        }
    }

    pub fn decl(&self, d: TypedDecl) -> MLDecl {
        match d {
            TypedDecl::Var(v) => MLDecl::Var(self.var(v)),
            TypedDecl::Fun(f) => MLDecl::Fun(self.fun(f)),
            TypedDecl::Struct(s) => MLDecl::Struct(self.struct_(s)),
            TypedDecl::Class => exit(-1),
            TypedDecl::Enum => exit(-1),
            TypedDecl::Protocol => exit(-1),
            TypedDecl::Extension => exit(-1),
        }
    }

    pub fn var(&self, v: TypedVar) -> MLVar {
        let expr = self.expr(v.value);
        MLVar {
            is_mute: v.is_mut,
            name: v.name,
            type_: self.type_(v.type_.unwrap()),
            value: expr,
        }
    }

    pub fn fun(&self, f: TypedFun) -> MLFun {
        let TypedFun {
            modifiers,
            name,
            arg_defs,
            body,
            return_type,
        } = f;
        let args = arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        MLFun {
            modifiers,
            name,
            arg_defs: args,
            return_type: self.type_(return_type),
            body: body.map(|b| self.fun_body(b)),
        }
    }

    pub fn struct_(&self, s: TypedStruct) -> MLStruct {
        MLStruct {
            name: s.name,
            fields: s
                .stored_properties
                .into_iter()
                .map(|p| MLField {
                    name: p.name,
                    type_: self.type_(p.type_),
                })
                .collect(),
        }
    }

    pub fn expr(&self, e: TypedExpr) -> MLExpr {
        match e {
            TypedExpr::Name(name) => MLExpr::Name(self.name(name)),
            TypedExpr::Literal(l) => MLExpr::Literal(self.literal(l)),
            TypedExpr::BinOp {
                left,
                kind,
                right,
                type_,
            } => MLExpr::PrimitiveBinOp(MLBinOp {
                left: Box::new(self.expr(*left)),
                kind: match &*kind {
                    "+" => MLBinopKind::Plus,
                    "-" => MLBinopKind::Minus,
                    "*" => MLBinopKind::Mul,
                    "/" => MLBinopKind::Div,
                    "%" => MLBinopKind::Mod,
                    "==" => MLBinopKind::Equal,
                    ">=" => MLBinopKind::GrateThanEqual,
                    ">" => MLBinopKind::GrateThan,
                    "<=" => MLBinopKind::LessThanEqual,
                    "<" => MLBinopKind::LessThan,
                    "!=" => MLBinopKind::NotEqual,
                    _ => exit(-1),
                },
                right: Box::new(self.expr(*right)),
                type_: self.type_(type_.unwrap()),
            }),
            TypedExpr::UnaryOp { .. } => exit(-1),
            TypedExpr::Subscript => exit(-1),
            TypedExpr::List => exit(-1),
            TypedExpr::Tuple => exit(-1),
            TypedExpr::Dict => exit(-1),
            TypedExpr::StringBuilder => exit(-1),
            TypedExpr::Call {
                target,
                args,
                type_,
            } => MLExpr::Call(MLCall {
                target: Box::new(self.expr(*target)),
                args: args
                    .into_iter()
                    .map(|a| MLCallArg {
                        arg: self.expr(*a.arg),
                    })
                    .collect(),
                type_: self.type_(type_.unwrap()),
            }),
            TypedExpr::If(i) => MLExpr::If(self.if_expr(i)),
            TypedExpr::When => exit(-1),
            TypedExpr::Lambda => exit(-1),
            TypedExpr::Return(r) => MLExpr::Return(self.return_expr(r)),
            TypedExpr::TypeCast => exit(-1),
        }
    }

    pub fn name(&self, n: TypedName) -> MLName {
        println!("{:?}", &n);
        MLName {
            name: n.name,
            type_: self.type_(n.type_.unwrap()),
        }
    }

    pub fn literal(&self, l: TypedLiteral) -> MLLiteral {
        match l {
            TypedLiteral::Integer { value, type_ } => MLLiteral::Integer {
                value: value,
                type_: self.type_(type_),
            },
            TypedLiteral::FloatingPoint { value, type_ } => MLLiteral::FloatingPoint {
                value: value,
                type_: self.type_(type_),
            },
            TypedLiteral::String { value, type_ } => MLLiteral::String {
                value: value,
                type_: self.type_(type_),
            },
            TypedLiteral::Boolean { value, type_ } => MLLiteral::Boolean {
                value: value,
                type_: self.type_(type_),
            },
            TypedLiteral::NullLiteral { type_ } => MLLiteral::Null {
                type_: self.type_(type_),
            },
        }
    }

    pub fn if_expr(&self, i: TypedIf) -> MLIf {
        MLIf {
            condition: Box::new(self.expr(*i.condition)),
            body: self.block(i.body),
            else_body: i.else_body.map(|b| self.block(b)),
            type_: self.type_(i.type_.unwrap()),
        }
    }

    pub fn return_expr(&self, r: TypedReturn) -> MLReturn {
        MLReturn {
            value: r.value.map(|v| Box::new(self.expr(*v))),
            type_: self.type_(r.type_.unwrap()),
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
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn::new(self.expr(e))))],
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
