use super::node::Node;
use crate::ast::expr::Expr;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::type_name::TypeName;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Decl {
    Var(VarSyntax),
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
    },
}

impl Node for Decl {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypeName>,
    pub(crate) value: Expr,
}
