use crate::high_level_ir::typed_decl::TypedStruct;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedExpr {
    Name(TypedName),
    Literal(TypedLiteral),
    BinOp(TypedBinOp),
    UnaryOp {
        target: Box<TypedExpr>,
        prefix: bool,
        kind: String,
        type_: Option<TypedType>,
    },
    Subscript(TypedSubscript),
    Member(TypedInstanceMember),
    StaticMember(TypedStaticMember),
    List,
    Tuple,
    Dict,
    StringBuilder,
    Call(TypedCall),
    If(TypedIf),
    When,
    Lambda,
    Return(TypedReturn),
    TypeCast,
    Type(TypedType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedName {
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedSubscript {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) indexes: Vec<TypedExpr>,
    pub(crate) type_: Option<TypedType>,
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
pub struct TypedBinOp {
    pub(crate) left: Box<TypedExpr>,
    pub(crate) kind: String,
    pub(crate)right: Box<TypedExpr>,
    pub(crate)type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedCall {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) args: Vec<TypedCallArg>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedCallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<TypedExpr>,
    pub(crate) is_vararg: bool,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedInstanceMember {
    pub(crate) target: Box<TypedExpr>,
    pub(crate) name: String,
    pub(crate) is_safe: bool,
    pub(crate) type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedStaticMember {
    pub(crate) target: TypedType,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedIf {
    pub(crate) condition: Box<TypedExpr>,
    pub(crate) body: TypedBlock,
    pub(crate) else_body: Option<TypedBlock>,
    pub(crate) type_: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedReturn {
    pub(crate) value: Option<Box<TypedExpr>>,
    pub(crate) type_: Option<TypedType>,
}

impl TypedExpr {
    pub fn type_(&self) -> Option<TypedType> {
        match self {
            TypedExpr::Name(name) => name.type_.clone(),
            TypedExpr::Literal(l) => Some(l.type_()),
            TypedExpr::BinOp(b) => b.type_.clone(),
            TypedExpr::UnaryOp {
                target,
                prefix,
                kind,
                type_,
            } => type_.clone(),
            TypedExpr::Subscript(s) => s.type_.clone(),
            TypedExpr::Member(m) => m.type_.clone(),
            TypedExpr::StaticMember(sm) => sm.type_.clone(),
            TypedExpr::List => None,
            TypedExpr::Tuple => None,
            TypedExpr::Dict => None,
            TypedExpr::StringBuilder => None,
            TypedExpr::Call(c) => c.type_.clone(),
            TypedExpr::If(i) => i.type_.clone(),
            TypedExpr::When => None,
            TypedExpr::Lambda => None,
            TypedExpr::Return(r) => r.type_.clone(),
            TypedExpr::TypeCast => None,
            TypedExpr::Type(_) => None,
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
