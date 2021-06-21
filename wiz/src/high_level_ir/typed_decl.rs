use std::fmt;
use crate::high_level_ir::typed_type::TypedType;
use crate::high_level_ir::typed_expr::TypedExpr;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedDecl {
    Var {
        is_mut: bool,
        name: String,
        type_: TypedType,
        value: TypedExpr
    },
    Fun {
        modifiers: Vec<String>,
        name: String,
        arg_defs: Vec<TypedArgDef>,
        body: Option<TypedFunBody>
    },
    Struct,
    Class,
    Enum,
    Protocol,
    Extension
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedArgDef {
    label: String,
    name: String,
    type_: TypedType
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedFunBody {
    // TODO
}
