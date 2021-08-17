use super::node::Node;
use crate::ast::expr::Expr;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::type_name::{TypeName, TypeParam};
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Decl {
    Var(VarSyntax),
    Fun(FunSyntax),
    Struct(StructSyntax),
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct FunSyntax {
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypeParam>>,
    pub(crate) arg_defs: Vec<ArgDef>,
    pub(crate) return_type: Option<TypeName>,
    pub(crate) body: Option<FunBody>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct StructSyntax {
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypeParam>>,
    pub(crate) properties: Vec<StructPropertySyntax>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum StructPropertySyntax {
    StoredProperty(StoredPropertySyntax),
    ComputedProperty,
    Init(InitializerSyntax),
    Method(MethodSyntax),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct StoredPropertySyntax {
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: TypeName,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct InitializerSyntax {
    pub(crate) args: Vec<ArgDef>,
    pub(crate) body: FunBody,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MethodSyntax {
    pub(crate) name: String,
    pub(crate) args: Vec<ArgDef>,
    pub(crate) type_params: Option<Vec<TypeParam>>,
    pub(crate) body: Option<FunBody>,
    pub(crate) return_type: Option<TypeName>,
}
