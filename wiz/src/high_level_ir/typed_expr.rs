use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedExpr {
    Name {
        name: String,
        type_: Option<TypedType>,
    },
    Literal(TypedLiteral),
    BinOp {
        left: Box<TypedExpr>,
        kind: String,
        right: Box<TypedExpr>,
        type_: Option<TypedType>,
    },
    UnaryOp {
        target: Box<TypedExpr>,
        prefix: bool,
        kind: String,
        type_: Option<TypedType>,
    },
    Subscript,
    List,
    Tuple,
    Dict,
    StringBuilder,
    Call {
        target: Box<TypedExpr>,
        args: Vec<TypedCallArg>,
        type_: Option<TypedType>,
    },
    If(TypedIf),
    When,
    Lambda,
    Return,
    TypeCast,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedLiteral {
    Integer { value: String, type_: TypedType },
    FloatingPoint { value: String, type_: TypedType },
    String { value: String, type_: TypedType },
    Boolean { value: String, type_: TypedType },
    NullLiteral { type_: TypedType },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedCallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<TypedExpr>,
    pub(crate) is_vararg: bool,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedIf {
    pub(crate) condition: Box<TypedExpr>,
    pub(crate) body: TypedBlock,
    pub(crate) else_body: Option<TypedBlock>,
    pub(crate) type_: Option<TypedType>,
}

impl TypedExpr {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedExpr::Name { name, type_ } => type_.clone(),
            TypedExpr::Literal(l) => Some(l.type_()),
            TypedExpr::BinOp {
                left,
                kind,
                right,
                type_,
            } => type_.clone(),
            TypedExpr::UnaryOp {
                target,
                prefix,
                kind,
                type_,
            } => type_.clone(),
            TypedExpr::Subscript => None,
            TypedExpr::List => None,
            TypedExpr::Tuple => None,
            TypedExpr::Dict => None,
            TypedExpr::StringBuilder => None,
            TypedExpr::Call {
                target,
                args,
                type_,
            } => type_.clone(),
            TypedExpr::If(i) => i.type_.clone(),
            TypedExpr::When => None,
            TypedExpr::Lambda => None,
            TypedExpr::Return => None,
            TypedExpr::TypeCast => None,
        }
    }
}

impl TypedLiteral {
    pub fn type_(&self) -> TypedType {
        match self {
            TypedLiteral::Integer { value, type_ } => type_.clone(),
            TypedLiteral::FloatingPoint { value, type_ } => type_.clone(),
            TypedLiteral::String { value, type_ } => type_.clone(),
            TypedLiteral::Boolean { value, type_ } => type_.clone(),
            TypedLiteral::NullLiteral { type_ } => type_.clone(),
        }
    }
}
