use super::node::Node;
use std::fmt;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::type_name::TypeName;
use crate::ast::block::Block;
use crate::ast::fun::body_def::FunBody;
use crate::ast::expr::Expr;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Decl {
    Var {
        is_mut: bool,
        name: String,
        type_: Option<TypeName>,
        value: Expr
    },
    Fun {
        modifiers: Vec<String>,
        name: String,
        arg_defs: Vec<ArgDef>,
        return_type: TypeName,
        body: Option<FunBody>,
    },
    Struct {
        // TODO
    },
    Class {
        // TODO
    },
    Enum {
        // TODO
    },
    Protocol {
        // TODO
    },
    Extension {
        // TODO
    }
}

impl Node for Decl {

}
