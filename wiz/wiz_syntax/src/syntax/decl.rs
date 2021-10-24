use fun_syntax::arg_def::ArgDef;
use fun_syntax::body_def::FunBody;

use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::decl::fun_syntax::FunSyntax;
use crate::syntax::decl::var_syntax::VarSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeName, TypeParameterListSyntax};
use crate::syntax::Syntax;

pub mod fun_syntax;
pub mod var_syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
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
            Decl::Var(v) => Decl::Var(v.with_annotation(a)),
            Decl::Fun(f) => Decl::Fun(f.with_annotation(a)),
            Decl::Struct(s) => Decl::Struct(s.with_annotation(a)),
            Decl::ExternC(e) => Decl::ExternC(e).with_annotation(a),
            Decl::Enum { .. } => Decl::Enum {},
            Decl::Protocol { .. } => Decl::Protocol {},
            Decl::Extension { .. } => Decl::Extension {},
            Decl::Use(u) => Decl::Use(u.with_annotation(a)),
        }
    }
}

impl Syntax for Decl {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub struct_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub properties: Vec<StructPropertySyntax>,
}

impl Annotatable for StructSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StructPropertySyntax {
    StoredProperty(StoredPropertySyntax),
    ComputedProperty,
    Init(InitializerSyntax),
    Deinit(DeinitializerSyntax),
    Method(MethodSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StoredPropertySyntax {
    pub is_mut: bool,
    pub name: String,
    pub type_: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InitializerSyntax {
    pub init_keyword: TokenSyntax,
    pub args: Vec<ArgDef>,
    pub body: FunBody,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DeinitializerSyntax {
    pub deinit_keyword: TokenSyntax,
    pub body: FunBody,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MethodSyntax {
    pub name: String,
    pub args: Vec<ArgDef>,
    pub type_params: Option<TypeParameterListSyntax>,
    pub body: Option<FunBody>,
    pub return_type: Option<TypeName>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UseSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub package_name: PackageName,
    pub alias: Option<String>,
}

impl Annotatable for UseSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PackageName {
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExternCSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub extern_keyword: TokenSyntax,
    pub left_brace: TokenSyntax,
    pub declarations: Vec<Decl>,
    pub right_brace: TokenSyntax,
}

impl Annotatable for ExternCSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}
