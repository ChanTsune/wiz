use crate::middle_level_ir::ml_stmt::MLBlock;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;
use std::process::exit;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLExpr {
    Name(MLName),
    Literal(MLLiteral),
    Call(MLCall),
    PrimitiveBinOp(MLBinOp),
    PrimitiveUnaryOp(MLUnaryOp),
    If(MLIf),
    When,
    Return(MLReturn),
    TypeCast,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLName {
    pub(crate) name: String,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLLiteral {
    Integer { value: String, type_: MLType },
    FloatingPoint { value: String, type_: MLType },
    String { value: String, type_: MLType },
    Boolean { value: String, type_: MLType },
    Null { type_: MLType },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLCall {
    pub(crate) target: Box<MLExpr>,
    pub(crate) args: Vec<MLCallArg>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLCallArg {
    pub(crate) arg: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLIf {
    pub(crate) condition: Box<MLExpr>,
    pub(crate) body: MLBlock,
    pub(crate) else_body: Option<MLBlock>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLBinOp {
    pub(crate) left: Box<MLExpr>,
    pub(crate) kind: MLBinopKind,
    pub(crate) right: Box<MLExpr>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLBinopKind {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equal,
    GrateThanEqual,
    GrateThan,
    LessThanEqual,
    LessThan,
    NotEqual,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLUnaryOp {
    pub(crate) target: Box<MLExpr>,
    pub(crate) kind: MLUnaryOpKind,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLUnaryOpKind {
    Negative,
    Positive,
    Not,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLReturn {
    pub(crate) value: Option<Box<MLExpr>>,
    pub(crate) type_: MLType,
}

impl MLExpr {
    pub fn type_(&self) -> MLType {
        match self {
            MLExpr::Name(n) => n.type_.clone(),
            MLExpr::Literal(l) => l.type_(),
            MLExpr::Call(c) => c.type_.clone(),
            MLExpr::PrimitiveBinOp(b) => b.type_.clone(),
            MLExpr::PrimitiveUnaryOp(b) => b.type_.clone(),
            MLExpr::If(i) => i.type_.clone(),
            MLExpr::When => exit(-9),
            MLExpr::Return(r) => r.type_.clone(),
            MLExpr::TypeCast => exit(-9),
        }
    }
}

impl MLLiteral {
    pub fn type_(&self) -> MLType {
        match self {
            MLLiteral::Integer { value, type_ } => type_.clone(),
            MLLiteral::FloatingPoint { value, type_ } => type_.clone(),
            MLLiteral::String { value, type_ } => type_.clone(),
            MLLiteral::Boolean { value, type_ } => type_.clone(),
            MLLiteral::Null { type_ } => type_.clone(),
        }
    }
}

impl MLCallArg {
    pub fn type_(&self) -> MLType {
        self.arg.type_()
    }
}

impl MLReturn {
    pub fn new(expr: MLExpr) -> Self {
        let type_ = expr.type_();
        MLReturn {
            value: Some(Box::new(expr)),
            type_: type_,
        }
    }
}
