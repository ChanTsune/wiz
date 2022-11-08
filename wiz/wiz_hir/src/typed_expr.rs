use crate::typed_stmt::TypedBlock;
use crate::typed_type::{TypedPackage, TypedType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: Option<TypedType>,
}

impl TypedExpr {
    pub fn new(kind: TypedExprKind, ty: Option<TypedType>) -> Self {
        Self { kind, ty }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExprKind {
    Name(TypedName),
    Literal(TypedLiteralKind),
    BinOp(TypedBinOp),
    UnaryOp(TypedUnaryOp),
    Subscript(TypedSubscript),
    Member(TypedInstanceMember),
    Array(TypedArray),
    Tuple,
    Dict,
    StringBuilder,
    Call(TypedCall),
    If(TypedIf),
    When,
    Lambda(TypedLambda),
    Return(TypedReturn),
    TypeCast(TypedTypeCast),
    SizeOf(TypedType),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedName {
    pub package: TypedPackage,
    pub name: String,
    pub type_arguments: Option<Vec<TypedType>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedArray {
    pub elements: Vec<TypedExpr>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedSubscript {
    pub target: Box<TypedExpr>,
    pub indexes: Vec<TypedExpr>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedLiteralKind {
    Integer(String),
    FloatingPoint(String),
    String(String),
    Boolean(String),
    NullLiteral,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedBinOp {
    pub left: Box<TypedExpr>,
    pub operator: TypedBinaryOperator,
    pub right: Box<TypedExpr>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum TypedBinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    GrateThanEqual,
    GrateThan,
    LessThanEqual,
    LessThan,
    NotEqual,
    And,
    Or,
    InfixFunctionCall(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedUnaryOp {
    Prefix(TypedPrefixUnaryOp),
    Postfix(TypedPostfixUnaryOp),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedPrefixUnaryOp {
    pub target: Box<TypedExpr>,
    pub operator: TypedPrefixUnaryOperator,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedPrefixUnaryOperator {
    Negative,
    Positive,
    Not,
    Reference,
    Dereference,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedPostfixUnaryOp {
    pub target: Box<TypedExpr>,
    pub operator: TypedPostfixUnaryOperator,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedPostfixUnaryOperator {
    Unwrap,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedCall {
    pub target: Box<TypedExpr>,
    pub args: Vec<TypedCallArg>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedCallArg {
    pub label: Option<String>,
    pub arg: Box<TypedExpr>,
    pub is_vararg: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedInstanceMember {
    pub target: Box<TypedExpr>,
    pub name: String,
    pub is_safe: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStaticMember {
    pub target: TypedType,
    pub name: String,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedIf {
    pub condition: Box<TypedExpr>,
    pub body: TypedBlock,
    pub else_body: Option<TypedBlock>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedLambda {
    pub args: Vec<String>,
    pub body: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedReturn {
    pub value: Option<Box<TypedExpr>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedTypeCast {
    pub target: Box<TypedExpr>,
    pub is_safe: bool,
    pub type_: TypedType,
}

impl TypedLiteralKind {
    pub fn is_integer(&self) -> bool {
        matches!(self, TypedLiteralKind::Integer { .. })
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, TypedLiteralKind::FloatingPoint { .. })
    }

    pub fn is_string(&self) -> bool {
        matches!(self, TypedLiteralKind::String { .. })
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, TypedLiteralKind::Boolean { .. })
    }

    pub fn is_null(&self) -> bool {
        matches!(self, TypedLiteralKind::NullLiteral { .. })
    }
}
