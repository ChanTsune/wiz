use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedCall, TypedExpr, TypedIf, TypedInstanceMember, TypedLiteral, TypedName, TypedReturn,
    TypedStaticMember,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt};
use crate::high_level_ir::typed_type::{TypedFunctionType, TypedType, TypedValueType};
use crate::middle_level_ir::ml_decl::{
    MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar,
};
use crate::middle_level_ir::ml_expr::{
    MLBinOp, MLBinopKind, MLCall, MLCallArg, MLExpr, MLIf, MLLiteral, MLMember, MLName, MLReturn,
};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLAssignmentStmt, MLBlock, MLLoopStmt, MLStmt};
use crate::middle_level_ir::ml_type::{MLFunctionType, MLType, MLValueType};
use std::collections::HashMap;
use std::process::exit;

pub mod ml_decl;
pub mod ml_expr;
pub mod ml_file;
pub mod ml_stmt;
pub mod ml_type;

pub struct HLIR2MLIR {
    structs: HashMap<MLValueType, MLStruct>,
}

impl HLIR2MLIR {
    pub fn new() -> Self {
        HLIR2MLIR {
            structs: HashMap::new(),
        }
    }

    fn get_struct(&self, typ: &MLType) -> &MLStruct {
        let typ = typ.clone();
        self.structs.get(&typ.into_value_type()).unwrap()
    }

    fn add_struct(&mut self, typ: MLValueType, struct_: MLStruct) {
        self.structs.insert(typ, struct_);
    }

    pub fn type_(&self, t: TypedType) -> MLType {
        match t {
            TypedType::Value(t) => MLType::Value(self.value_type(t)),
            TypedType::Function(f) => MLType::Function(self.function_type(*f)),
        }
    }

    pub fn value_type(&self, t: TypedValueType) -> MLValueType {
        let mut pkg = t.package.names;
        pkg.push(t.name);
        MLValueType {
            name: pkg.join("::"),
        }
    }

    pub fn function_type(&self, t: TypedFunctionType) -> MLFunctionType {
        MLFunctionType {
            arguments: t
                .arguments
                .into_iter()
                .map(|a| match self.type_(a.type_) {
                    MLType::Value(v) => v,
                    MLType::Function(_) => exit(-9),
                })
                .collect(),
            return_type: match self.type_(t.return_type) {
                MLType::Value(v) => v,
                MLType::Function(f) => {
                    println!("{:?}", f);
                    exit(-9)
                }
            },
        }
    }

    pub fn file(&mut self, f: TypedFile) -> MLFile {
        MLFile {
            body: f.body.into_iter().map(|d| self.decl(d)).flatten().collect(),
        }
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Vec<MLStmt> {
        match s {
            TypedStmt::Expr(e) => vec![MLStmt::Expr(self.expr(e))],
            TypedStmt::Decl(d) => self
                .decl(d)
                .into_iter()
                .map(|dc| MLStmt::Decl(dc))
                .collect(),
            TypedStmt::Assignment(a) => vec![MLStmt::Assignment(self.assignment(a))],
            TypedStmt::Loop(l) => vec![MLStmt::Loop(self.loop_stmt(l))],
        }
    }

    pub fn assignment(&mut self, a: TypedAssignmentStmt) -> MLAssignmentStmt {
        match a {
            TypedAssignmentStmt::Assignment(a) => MLAssignmentStmt {
                target: self.expr(a.target),
                value: self.expr(a.value),
            },
        }
    }

    pub fn loop_stmt(&mut self, l: TypedLoopStmt) -> MLLoopStmt {
        match l {
            TypedLoopStmt::While(w) => MLLoopStmt {
                condition: self.expr(w.condition),
                block: self.block(w.block),
            },
            TypedLoopStmt::For(_) => exit(-1),
        }
    }

    pub fn decl(&mut self, d: TypedDecl) -> Vec<MLDecl> {
        match d {
            TypedDecl::Var(v) => vec![MLDecl::Var(self.var(v))],
            TypedDecl::Fun(f) => vec![MLDecl::Fun(self.fun(f))],
            TypedDecl::Struct(s) => {
                let (st, fns) = self.struct_(s);
                let mut fns: Vec<MLDecl> = fns.into_iter().map(|f| MLDecl::Fun(f)).collect();
                let mut r = vec![MLDecl::Struct(st)];
                r.append(&mut fns);
                r
            }
            TypedDecl::Class => exit(-1),
            TypedDecl::Enum => exit(-1),
            TypedDecl::Protocol => exit(-1),
            TypedDecl::Extension => exit(-1),
        }
    }

    pub fn var(&mut self, v: TypedVar) -> MLVar {
        let expr = self.expr(v.value);
        MLVar {
            is_mute: v.is_mut,
            name: v.name,
            type_: self.type_(v.type_.unwrap()),
            value: expr,
        }
    }

    pub fn fun(&mut self, f: TypedFun) -> MLFun {
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

    pub fn struct_(&mut self, s: TypedStruct) -> (MLStruct, Vec<MLFun>) {
        let TypedStruct {
            name,
            init,
            stored_properties,
            computed_properties,
            member_functions,
            static_function,
        } = s;
        let struct_ = MLStruct {
            name: name.clone(),
            fields: stored_properties
                .into_iter()
                .map(|p| MLField {
                    name: p.name,
                    type_: self.type_(p.type_),
                })
                .collect(),
        };

        self.add_struct(
            MLValueType {
                name: struct_.name.clone(),
            },
            struct_.clone(),
        );

        let mut init: Vec<MLFun> = init
            .into_iter()
            .map(|i| {
                let type_ = self.type_(i.type_);
                let mut body = self.block(i.block).body;
                body.insert(
                    0,
                    MLStmt::Decl(MLDecl::Var(MLVar {
                        is_mute: true,
                        name: String::from("self"),
                        value: MLExpr::Literal(MLLiteral::Struct {
                            type_: type_.clone(),
                        }),
                        type_: type_.clone(),
                    })),
                );
                MLFun {
                    modifiers: vec![],
                    name: name.clone() + ".init",
                    arg_defs: i.args.into_iter().map(|a| self.arg_def(a)).collect(),
                    return_type: type_,
                    body: Some(MLFunBody { body }),
                }
            })
            .collect();
        let mut funs: Vec<MLFun> = vec![];
        funs.append(&mut init);
        (struct_, funs)
    }

    pub fn expr(&mut self, e: TypedExpr) -> MLExpr {
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
            TypedExpr::Member(m) => self.member(m),
            TypedExpr::StaticMember(sm) => self.static_member(sm),
            TypedExpr::List => exit(-1),
            TypedExpr::Tuple => exit(-1),
            TypedExpr::Dict => exit(-1),
            TypedExpr::StringBuilder => exit(-1),
            TypedExpr::Call(c) => MLExpr::Call(self.call(c)),
            TypedExpr::If(i) => MLExpr::If(self.if_expr(i)),
            TypedExpr::When => exit(-1),
            TypedExpr::Lambda => exit(-1),
            TypedExpr::Return(r) => MLExpr::Return(self.return_expr(r)),
            TypedExpr::TypeCast => exit(-1),
            TypedExpr::Type(t) => {
                eprintln!("Never execution branch executed!! => {:?}", t);
                exit(-1)
            }
        }
    }

    pub fn name(&self, n: TypedName) -> MLName {
        println!("MLIR => name :: {:?}", &n);
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

    pub fn member(&mut self, m: TypedInstanceMember) -> MLExpr {
        let TypedInstanceMember {
            target,
            name,
            is_safe,
            type_,
        } = m;
        let target = self.expr(*target);
        let struct_ = self.get_struct(&target.type_());
        let type_ = self.type_(type_.unwrap());
        let is_stored = struct_.fields.iter().any(|f| f.name == name);
        if is_stored {
            MLExpr::Member(MLMember {
                target: Box::new(target),
                name,
                type_,
            })
        } else {
            MLExpr::Call(MLCall {
                target: Box::new(MLExpr::Name(MLName {
                    name: target.type_().into_value_type().name + "." + &*name,
                    type_: type_.clone(),
                })),
                args: vec![],
                type_: type_,
            })
        }
        // else field as function call etc...
    }

    pub fn static_member(&self, sm: TypedStaticMember) -> MLExpr {
        let type_name = self.type_(sm.target).into_value_type().name;
        MLExpr::Name(MLName {
            name: type_name + "#" + &*sm.name,
            type_: self.type_(sm.type_.unwrap()),
        })
    }

    pub fn call(&mut self, c: TypedCall) -> MLCall {
        let TypedCall {
            target,
            args,
            type_,
        } = c;
        MLCall {
            target: Box::new(self.expr(*target)),
            args: args
                .into_iter()
                .map(|a| MLCallArg {
                    arg: self.expr(*a.arg),
                })
                .collect(),
            type_: self.type_(type_.unwrap()),
        }
    }

    pub fn if_expr(&mut self, i: TypedIf) -> MLIf {
        MLIf {
            condition: Box::new(self.expr(*i.condition)),
            body: self.block(i.body),
            else_body: i.else_body.map(|b| self.block(b)),
            type_: self.type_(i.type_.unwrap()),
        }
    }

    pub fn return_expr(&mut self, r: TypedReturn) -> MLReturn {
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

    pub fn fun_body(&mut self, b: TypedFunBody) -> MLFunBody {
        match b {
            TypedFunBody::Expr(e) => MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn::new(self.expr(e))))],
            },
            TypedFunBody::Block(b) => MLFunBody {
                body: self.block(b).body,
            },
        }
    }

    pub fn block(&mut self, b: TypedBlock) -> MLBlock {
        MLBlock {
            body: b.body.into_iter().map(|s| self.stmt(s)).flatten().collect(),
        }
    }
}
