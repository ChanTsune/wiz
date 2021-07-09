use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedDecl {
    Var {
        is_mut: bool,
        name: String,
        type_: Option<TypedType>,
        value: TypedExpr,
    },
    Fun(TypedFun),
    Struct,
    Class,
    Enum,
    Protocol,
    Extension,
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
