use super::node::SyntaxNode;
use crate::syntax::expr::Expr;
use crate::syntax::fun::arg_def::ArgDef;
use crate::syntax::fun::body_def::FunBody;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{TypeName, TypeParam};
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Decl {
    Var(VarSyntax),
    Fun(FunSyntax),
    Struct(StructSyntax),
    ExternC(ExternCSyntax),
    Enum {
        // TODO
    },
    Protocol {
        // TODO
    },
    Extension {
        // TODO
    },
    Use(UseSyntax),
}

impl SyntaxNode for Decl {}

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
    pub(crate) annotations: Vec<String>, // TODO: Change to AnnotationSyntax type
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct UseSyntax {
    pub(crate) package_name: PackageName,
    pub(crate) alias: Option<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct PackageName {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ExternCSyntax {
    pub(crate) extern_keyword: TokenSyntax,
    pub(crate) left_brace: TokenSyntax,
    pub(crate) declarations: Vec<Decl>,
    pub(crate) right_brace: TokenSyntax,
}
