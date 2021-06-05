use super::node::Node;
use std::fmt;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::type_name::TypeName;
use crate::ast::block::Block;
use crate::ast::fun::body_def::FunBody;

#[derive(fmt::Debug, Eq, PartialEq)]
pub enum Decl {
    Var {
        // TODO
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
