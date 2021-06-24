use std::fmt;
use crate::high_level_ir::typed_type::TypedType;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedExpr {
    Name {
        name: String,
        type_: TypedType
    },
    Literal(TypedLiteral),
    BinOp {
        left: Box<TypedExpr>,
        kind: String,
        right: Box<TypedExpr>,
        type_: TypedType
    },
    UnaryOp {
        target: Box<TypedExpr>,
        prefix: bool,
        kind: String,
        type_: TypedType
    },
    Subscript,
    List,
    Tuple,
    Dict,
    StringBuilder,
    Call {
        target: Box<TypedExpr>,
        args: Vec<TypedCallArg>,
        type_: TypedType
    },
    If,
    When,
    Lambda,
    Return,
    TypeCast
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedLiteral {
    Integer {
        value: String,
        type_: TypedType
    },
    FloatingPoint {
        value: String,
        type_: TypedType
    },
    String {
        value: String,
        type_: TypedType
    },
    Boolean {
        value: String,
        type_: TypedType
    },
    NullLiteral {
        type_: TypedType
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedCallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<TypedExpr>,
    pub(crate) is_vararg: bool,
}


impl TypedExpr {
    pub fn type_(&self) -> TypedType {
        match self {
            TypedExpr::Name { name, type_ } => { type_.clone() }
            TypedExpr::Literal(l) => {l.type_()}
            TypedExpr::BinOp { left, kind, right, type_ } => {type_.clone()}
            TypedExpr::UnaryOp { target, prefix, kind, type_ } => {type_.clone()}
            TypedExpr::Subscript => {
                TypedType::noting()
            }
            TypedExpr::List => {
                TypedType::noting()
            }
            TypedExpr::Tuple => {
                TypedType::noting()
            }
            TypedExpr::Dict => {
                TypedType::noting()
            }
            TypedExpr::StringBuilder => {
                TypedType::noting()
            }
            TypedExpr::Call { target, args, type_ } => {type_.clone()}
            TypedExpr::If => {
                TypedType::noting()
            }
            TypedExpr::When => {
                TypedType::noting()
            }
            TypedExpr::Lambda => {
                TypedType::noting()
            }
            TypedExpr::Return => {
                TypedType::noting()
            }
            TypedExpr::TypeCast => {
                TypedType::noting()
            }
        }
    }
}

impl TypedLiteral {
    pub fn type_(&self) -> TypedType {
        match self {
            TypedLiteral::Integer { value, type_ } => {type_.clone()}
            TypedLiteral::FloatingPoint { value, type_ } => {type_.clone()}
            TypedLiteral::String { value, type_ } => {type_.clone()}
            TypedLiteral::Boolean { value, type_ } => {type_.clone()}
            TypedLiteral::NullLiteral { type_ } => {type_.clone()}
        }
    }
}