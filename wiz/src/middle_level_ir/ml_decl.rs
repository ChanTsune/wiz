use crate::ast::type_name::TypeName;
use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_type::MLType;

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
        return_type: TypeName,
        body: Option<MLFunBody>,
    },
    Struct,
}

pub struct MLArgDef {
    name: String,
    type_: MLType,
}

pub struct MLFunBody {}
