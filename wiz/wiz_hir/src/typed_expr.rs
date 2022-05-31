use crate::typed_stmt::TypedBlock;
use crate::typed_type::{TypedPackage, TypedType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExprKind {
    Name(TypedName),
    Literal(TypedLiteralKind, Option<TypedType>),
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
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedName {
    pub package: TypedPackage,
    pub name: String,
    pub type_: Option<TypedType>,
    pub type_arguments: Option<Vec<TypedType>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedArray {
    pub elements: Vec<TypedExprKind>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedSubscript {
    pub target: Box<TypedExprKind>,
    pub indexes: Vec<TypedExprKind>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedLiteralKind {
    Integer {
        value: String,
    },
    FloatingPoint {
        value: String,
    },
    String {
        value: String,
    },
    Boolean {
        value: String,
    },
    NullLiteral,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedBinOp {
    pub left: Box<TypedExprKind>,
    pub operator: TypedBinaryOperator,
    pub right: Box<TypedExprKind>,
    pub type_: Option<TypedType>,
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
    InfixFunctionCall(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedUnaryOp {
    Prefix(TypedPrefixUnaryOp),
    Postfix(TypedPostfixUnaryOp),
}

impl TypedUnaryOp {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedUnaryOp::Prefix(p) => p.type_.clone(),
            TypedUnaryOp::Postfix(p) => p.type_.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedPrefixUnaryOp {
    pub target: Box<TypedExprKind>,
    pub operator: TypedPrefixUnaryOperator,
    pub type_: Option<TypedType>,
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
    pub target: Box<TypedExprKind>,
    pub operator: TypedPostfixUnaryOperator,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedPostfixUnaryOperator {
    Unwrap,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedCall {
    pub target: Box<TypedExprKind>,
    pub args: Vec<TypedCallArg>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedCallArg {
    pub label: Option<String>,
    pub arg: Box<TypedExprKind>,
    pub is_vararg: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedInstanceMember {
    pub target: Box<TypedExprKind>,
    pub name: String,
    pub is_safe: bool,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStaticMember {
    pub target: TypedType,
    pub name: String,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedIf {
    pub condition: Box<TypedExprKind>,
    pub body: TypedBlock,
    pub else_body: Option<TypedBlock>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedLambda {
    pub args: Vec<String>,
    pub body: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedReturn {
    pub value: Option<Box<TypedExprKind>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedTypeCast {
    pub target: Box<TypedExprKind>,
    pub is_safe: bool,
    pub type_: Option<TypedType>,
}

impl TypedExprKind {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedExprKind::Name(name) => name.type_.clone(),
            TypedExprKind::Literal(l, type_) => type_.clone(),
            TypedExprKind::BinOp(b) => b.type_.clone(),
            TypedExprKind::UnaryOp(u) => u.type_(),
            TypedExprKind::Subscript(s) => s.type_.clone(),
            TypedExprKind::Member(m) => m.type_.clone(),
            TypedExprKind::Array(a) => a.type_.clone(),
            TypedExprKind::Tuple => None,
            TypedExprKind::Dict => None,
            TypedExprKind::StringBuilder => None,
            TypedExprKind::Call(c) => c.type_.clone(),
            TypedExprKind::If(i) => i.type_.clone(),
            TypedExprKind::When => None,
            TypedExprKind::Lambda(l) => todo!(),
            TypedExprKind::Return(r) => Some(r.type_()),
            TypedExprKind::TypeCast(t) => t.type_.clone(),
        }
    }
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

impl TypedReturn {
    pub fn type_(&self) -> TypedType {
        TypedType::noting()
    }
}
