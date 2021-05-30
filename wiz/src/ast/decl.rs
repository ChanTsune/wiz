use super::node::Node;
use std::fmt;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::type_name::TypeName;
use crate::ast::block::Block;

#[derive(fmt::Debug)]
pub enum Decl {
    Var {
        // TODO
    },
    Fun {
        name: String,
        arg_defs: Vec<ArgDef>,
        return_type: TypeName,
        body: Block,
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
