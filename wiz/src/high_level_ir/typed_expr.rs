use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::{Package, TypedType};

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedName {
    pub(crate) package: Option<Package>,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedArray {
    pub(crate) elements: Vec<TypedExpr>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedSubscript {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) indexes: Vec<TypedExpr>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedBinOp {
    pub(crate) left: Box<TypedExpr>,
    pub(crate) operator: TypedBinaryOperator,
    pub(crate) right: Box<TypedExpr>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedUnaryOp {
    Prefix(TypedPrefixUnaryOp),
    Postfix(TypedPostfixUnaryOp),
}

impl TypedUnaryOp {
    pub(crate) fn type_(&self) -> Option<TypedType> {
        match self {
            TypedUnaryOp::Prefix(p) => p.type_.clone(),
            TypedUnaryOp::Postfix(p) => p.type_.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedPrefixUnaryOp {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) operator: TypedPrefixUnaryOperator,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedPrefixUnaryOperator {
    Negative,
    Positive,
    Not,
    Reference,
    Dereference,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedPostfixUnaryOp {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) operator: TypedPostfixUnaryOperator,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedPostfixUnaryOperator {
    Unwrap,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedCall {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) args: Vec<TypedCallArg>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedCallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<TypedExpr>,
    pub(crate) is_vararg: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedInstanceMember {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) name: String,
    pub(crate) is_safe: bool,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedStaticMember {
    pub(crate) target: TypedType,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedIf {
    pub(crate) condition: Box<TypedExpr>,
    pub(crate) body: TypedBlock,
    pub(crate) else_body: Option<TypedBlock>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedLambda {
    pub(crate) args: Vec<String>,
    pub(crate) body: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedReturn {
    pub(crate) value: Option<Box<TypedExpr>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedTypeCast {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) is_safe: bool,
    pub(crate) type_: Option<TypedType>,
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
    pub(crate) fn type_(&self) -> TypedType {
        TypedType::noting()
    }
}
