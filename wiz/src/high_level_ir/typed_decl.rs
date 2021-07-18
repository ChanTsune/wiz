use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedDecl {
    Var(TypedVar),
    Fun(TypedFun),
    Struct(TypedStruct),
    Class,
    Enum,
    Protocol,
    Extension,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedVar {
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
    pub(crate) value: TypedExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedFun {
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) arg_defs: Vec<TypedArgDef>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedFunBody {
    Expr(TypedExpr),
    Block(TypedBlock),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedStruct {
    pub(crate) name: String,
    pub(crate) stored_properties: Vec<TypedStoredProperty>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedStoredProperty {
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}
