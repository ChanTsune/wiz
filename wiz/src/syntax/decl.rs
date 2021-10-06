use super::node::SyntaxNode;
use crate::syntax::expr::Expr;
use crate::syntax::fun::arg_def::ArgDef;
use crate::syntax::fun::body_def::FunBody;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{TypeName, TypeParam};
use std::fmt;
use crate::syntax::annotation::{AnnotationsSyntax, Annotatable};

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

impl Annotatable for Decl {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self {
        match self {
            Decl::Var(v) => {Decl::Var(v.with_annotation(a))}
            Decl::Fun(f) => {Decl::Fun(f.with_annotation(a))}
            Decl::Struct(s) => {Decl::Struct(s.with_annotation(a))}
            Decl::ExternC(e) => {Decl::ExternC(e).with_annotation(a)}
            Decl::Enum { .. } => Decl::Enum { },
            Decl::Protocol { .. } => Decl::Protocol {},
            Decl::Extension { .. } => Decl::Extension {},
            Decl::Use(u) => {Decl::Use(u.with_annotation(a))}
        }
    }
}

impl SyntaxNode for Decl {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub(crate) annotations: Option<AnnotationsSyntax>,
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypeName>,
    pub(crate) value: Expr,
}

impl Annotatable for VarSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct FunSyntax {
    pub(crate) annotations: Option<AnnotationsSyntax>,
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypeParam>>,
    pub(crate) arg_defs: Vec<ArgDef>,
    pub(crate) return_type: Option<TypeName>,
    pub(crate) body: Option<FunBody>,
}

impl Annotatable for FunSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct StructSyntax {
    pub(crate) annotations: Option<AnnotationsSyntax>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypeParam>>,
    pub(crate) properties: Vec<StructPropertySyntax>,
}

impl Annotatable for StructSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
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
    pub(crate) annotations: Option<AnnotationsSyntax>,
    pub(crate) package_name: PackageName,
    pub(crate) alias: Option<String>,
}

impl Annotatable for UseSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct PackageName {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ExternCSyntax {
    pub(crate) annotations: Option<AnnotationsSyntax>,
    pub(crate) extern_keyword: TokenSyntax,
    pub(crate) left_brace: TokenSyntax,
    pub(crate) declarations: Vec<Decl>,
    pub(crate) right_brace: TokenSyntax,
}

impl Annotatable for ExternCSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}
