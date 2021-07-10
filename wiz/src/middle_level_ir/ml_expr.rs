use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::middle_level_ir::ml_stmt::MLBlock;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLExpr {
    Name(MLName),
    Literal(MLLiteral),
    Call(MLCall),
    PrimitiveBinOp(MLBinOp),
    PrimitiveUnaryOp(MLUnaryOp),
    If(MLIf),
    When,
    Return,
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
