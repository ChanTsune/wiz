use crate::typed_stmt::TypedBlock;
use crate::typed_type::{TypedPackage, TypedType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpr {
    Name(TypedName),
    Literal(TypedLiteral),
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
    pub elements: Vec<TypedExpr>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedSubscript {
    pub target: Box<TypedExpr>,
    pub indexes: Vec<TypedExpr>,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedLiteral {
    Integer {
        value: String,
        type_: Option<TypedType>,
    },
    FloatingPoint {
        value: String,
        type_: Option<TypedType>,
    },
    String {
        value: String,
        type_: Option<TypedType>,
    },
    Boolean {
        value: String,
        type_: Option<TypedType>,
    },
    NullLiteral {
        type_: Option<TypedType>,
    },
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedBinOp {
    pub left: Box<TypedExpr>,
    pub operator: TypedBinaryOperator,
    pub right: Box<TypedExpr>,
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
    pub target: Box<TypedExpr>,
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
    pub target: Box<TypedExpr>,
    pub operator: TypedPostfixUnaryOperator,
    pub type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedPostfixUnaryOperator {
    Unwrap,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedCall {
    pub target: Box<TypedExpr>,
    pub args: Vec<TypedCallArg>,
    pub type_: Option<TypedType>,
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
    pub condition: Box<TypedExpr>,
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
    pub value: Option<Box<TypedExpr>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedTypeCast {
    pub target: Box<TypedExpr>,
    pub is_safe: bool,
    pub type_: Option<TypedType>,
}

impl TypedExpr {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedExpr::Name(name) => name.type_.clone(),
            TypedExpr::Literal(l) => l.type_(),
            TypedExpr::BinOp(b) => b.type_.clone(),
            TypedExpr::UnaryOp(u) => u.type_(),
            TypedExpr::Subscript(s) => s.type_.clone(),
            TypedExpr::Member(m) => m.type_.clone(),
            TypedExpr::Array(a) => a.type_.clone(),
            TypedExpr::Tuple => None,
            TypedExpr::Dict => None,
            TypedExpr::StringBuilder => None,
            TypedExpr::Call(c) => c.type_.clone(),
            TypedExpr::If(i) => i.type_.clone(),
            TypedExpr::When => None,
            TypedExpr::Lambda(l) => todo!(),
            TypedExpr::Return(r) => Some(r.type_()),
            TypedExpr::TypeCast(t) => t.type_.clone(),
        }
    }
}

impl TypedLiteral {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedLiteral::Integer { value: _, type_ } => type_.clone(),
            TypedLiteral::FloatingPoint { value: _, type_ } => type_.clone(),
            TypedLiteral::String { value: _, type_ } => type_.clone(),
            TypedLiteral::Boolean { value: _, type_ } => type_.clone(),
            TypedLiteral::NullLiteral { type_ } => type_.clone(),
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, TypedLiteral::Integer { .. })
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, TypedLiteral::FloatingPoint { .. })
    }

    pub fn is_string(&self) -> bool {
        matches!(self, TypedLiteral::String { .. })
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, TypedLiteral::Boolean { .. })
    }

    pub fn is_null(&self) -> bool {
        matches!(self, TypedLiteral::NullLiteral { .. })
    }
}

impl TypedReturn {
    pub fn type_(&self) -> TypedType {
        TypedType::noting()
    }
}
