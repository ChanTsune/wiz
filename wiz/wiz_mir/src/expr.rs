mod block;
mod if_expr;

pub use self::block::MLBlock;
pub use self::if_expr::MLIf;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::{MLType, MLValueType};
use crate::statement::MLReturn;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLExpr {
    Name(MLName),
    Literal(MLLiteral),
    Call(MLCall),
    PrimitiveBinOp(MLBinOp),
    PrimitiveUnaryOp(MLUnaryOp),
    PrimitiveSubscript(MLSubscript),
    Member(MLMember),
    If(MLIf),
    When,
    Return(MLReturn),
    PrimitiveTypeCast(MLTypeCast),
    Block(MLBlock),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLName {
    pub name: String,
    pub type_: MLType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLLiteral {
    Integer { value: String, type_: MLValueType },
    FloatingPoint { value: String, type_: MLValueType },
    String { value: String, type_: MLValueType },
    Boolean { value: String, type_: MLValueType },
    Null { type_: MLValueType },
    Struct { type_: MLValueType },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLCall {
    pub target: Box<MLExpr>,
    pub args: Vec<MLCallArg>,
    pub type_: MLType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLCallArg {
    pub arg: MLExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLBinOp {
    pub left: Box<MLExpr>,
    pub kind: MLBinOpKind,
    pub right: Box<MLExpr>,
    pub type_: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLBinOpKind {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLUnaryOp {
    pub target: Box<MLExpr>,
    pub kind: MLUnaryOpKind,
    pub type_: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLUnaryOpKind {
    Negative,
    Positive,
    Not,
    Ref,
    DeRef,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLSubscript {
    pub target: Box<MLExpr>,
    pub index: Box<MLExpr>,
    pub type_: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLMember {
    pub target: Box<MLExpr>,
    pub name: String,
    pub type_: MLType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLTypeCast {
    pub target: Box<MLExpr>,
    pub type_: MLValueType,
}

impl MLExpr {
    pub fn type_(&self) -> MLType {
        match self {
            MLExpr::Name(n) => n.type_.clone(),
            MLExpr::Literal(l) => MLType::Value(l.type_()),
            MLExpr::Call(c) => c.type_.clone(),
            MLExpr::PrimitiveBinOp(b) => MLType::Value(b.type_.clone()),
            MLExpr::PrimitiveUnaryOp(b) => MLType::Value(b.type_.clone()),
            MLExpr::PrimitiveSubscript(p) => MLType::Value(p.type_.clone()),
            MLExpr::Member(f) => f.type_.clone(),
            MLExpr::If(i) => MLType::Value(i.type_.clone()),
            MLExpr::When => todo!(),
            MLExpr::Return(r) => MLType::Value(r.type_()),
            MLExpr::PrimitiveTypeCast(t) => MLType::Value(t.type_.clone()),
            MLExpr::Block(b) => b.r#type(),
        }
    }
}

impl MLLiteral {
    pub fn type_(&self) -> MLValueType {
        match self {
            MLLiteral::Integer { value: _, type_ } => type_.clone(),
            MLLiteral::FloatingPoint { value: _, type_ } => type_.clone(),
            MLLiteral::String { value: _, type_ } => type_.clone(),
            MLLiteral::Boolean { value: _, type_ } => type_.clone(),
            MLLiteral::Null { type_ } => type_.clone(),
            MLLiteral::Struct { type_ } => type_.clone(),
        }
    }
}

impl MLCallArg {
    pub fn type_(&self) -> MLType {
        self.arg.type_()
    }
}

impl MLNode for MLExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLExpr::Name(n) => n.fmt(f),
            MLExpr::Literal(l) => l.fmt(f),
            MLExpr::Call(c) => c.fmt(f),
            MLExpr::PrimitiveBinOp(b) => b.fmt(f),
            MLExpr::PrimitiveUnaryOp(u) => u.fmt(f),
            MLExpr::PrimitiveSubscript(p) => p.fmt(f),
            MLExpr::Member(m) => m.fmt(f),
            MLExpr::If(i) => i.fmt(f),
            MLExpr::When => fmt::Result::Err(Default::default()),
            MLExpr::Return(r) => r.fmt(f),
            MLExpr::PrimitiveTypeCast(t) => t.fmt(f),
            MLExpr::Block(b) => b.fmt(f),
        }
    }
}

impl MLNode for MLName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)
    }
}

impl MLNode for MLLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLLiteral::Integer { value, type_: _ } => f.write_str(value),
            MLLiteral::FloatingPoint { value, type_: _ } => f.write_str(value),
            MLLiteral::String { value, type_: _ } => {
                f.write_char('"')?;
                f.write_str(value)?;
                f.write_char('"')
            }
            MLLiteral::Boolean { value, type_: _ } => f.write_str(value),
            MLLiteral::Null { type_: _ } => fmt::Result::Err(Default::default()),
            MLLiteral::Struct { type_ } => {
                type_.fmt(f)?;
                f.write_str(" { }")
            }
        }
    }
}

impl MLNode for MLCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('(')?;
        for (c, arg) in self.args.iter().enumerate() {
            arg.fmt(f)?;
            let s = self.args.len() - 1;
            if s != c {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

impl MLNode for MLCallArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.arg.fmt(f)
    }
}

impl MLNode for MLBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.left.fmt(f)?;
        f.write_char(' ')?;
        f.write_str(match self.kind {
            MLBinOpKind::Plus => "+",
            MLBinOpKind::Minus => "-",
            MLBinOpKind::Mul => "*",
            MLBinOpKind::Div => "/",
            MLBinOpKind::Mod => "%",
            MLBinOpKind::Equal => "==",
            MLBinOpKind::GrateThanEqual => "<=",
            MLBinOpKind::GrateThan => "<",
            MLBinOpKind::LessThanEqual => ">=",
            MLBinOpKind::LessThan => ">",
            MLBinOpKind::NotEqual => "!=",
        })?;
        f.write_char(' ')?;
        self.right.fmt(f)
    }
}

impl MLNode for MLUnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self.kind {
            MLUnaryOpKind::Negative => "-",
            MLUnaryOpKind::Positive => "+",
            MLUnaryOpKind::Not => "!",
            MLUnaryOpKind::Ref => "&",
            MLUnaryOpKind::DeRef => "*",
        })?;
        self.target.fmt(f)
    }
}

impl MLNode for MLSubscript {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('[')?;
        self.index.fmt(f)?;
        f.write_char(']')
    }
}

impl MLNode for MLMember {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('.')?;
        f.write_str(&*self.name)
    }
}

impl MLNode for MLTypeCast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_str(" as ")?;
        self.type_.fmt(f)
    }
}