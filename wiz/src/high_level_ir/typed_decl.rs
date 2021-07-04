use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedDecl {
    Var {
        is_mut: bool,
        name: String,
        type_: TypedType,
        value: TypedExpr,
    },
    Fun {
        modifiers: Vec<String>,
        name: String,
        arg_defs: Vec<TypedArgDef>,
        body: Option<TypedFunBody>,
        return_type: TypedType,
    },
    Struct,
    Class,
    Enum,
    Protocol,
    Extension,
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
