use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLDecl {
    Var {
        is_mute: bool,
        name: String,
        type_: MLType,
        value: MLExpr,
    },
    Fun {
        modifiers: Vec<String>,
        name: String,
        arg_defs: Vec<MLArgDef>,
        return_type: MLType,
        body: Option<MLFunBody>,
    },
    Struct,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLArgDef {
    name: String,
    type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFunBody {}
