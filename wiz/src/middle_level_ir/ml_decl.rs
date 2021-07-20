use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_stmt::MLStmt;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLDecl {
    Var(MLVar),
    Fun {
        modifiers: Vec<String>,
        name: String,
        arg_defs: Vec<MLArgDef>,
        return_type: MLType,
        body: Option<MLFunBody>,
    },
    Struct(MLStruct),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLVar  {
    pub(crate) is_mute: bool,
    pub(crate) name: String,
    pub(crate) type_: MLType,
    pub(crate) value: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLArgDef {
    pub(crate) name: String,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFunBody {
    pub(crate) body: Vec<MLStmt>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLStruct {
    pub(crate) name: String,
    pub(crate) fields: Vec<MLField>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLField {
    pub(crate) name: String,
    pub(crate) type_: MLType,
}
