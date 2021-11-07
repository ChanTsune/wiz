use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::declaration::fun_syntax::{ArgDefListSyntax, FunBody, FunSyntax};
pub use crate::syntax::declaration::use_syntax::{
    AliasSyntax, PackageName, PackageNameElement, UseSyntax,
};
pub use crate::syntax::declaration::var_syntax::VarSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeName, TypeParameterListSyntax};
use crate::syntax::Syntax;

pub mod fun_syntax;
mod use_syntax;
mod var_syntax;

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
        match self {
            Decl::Var(v) => Decl::Var(v.with_leading_trivia(trivia)),
            Decl::Fun(f) => {
                Decl::Fun(f.with_leading_trivia(trivia))
            }
            Decl::Struct(_) => {
                todo!()
            }
            Decl::ExternC(_) => {
                todo!()
            }
            Decl::Enum { .. } => {
                todo!()
            }
            Decl::Protocol { .. } => {
                todo!()
            }
            Decl::Extension { .. } => {
                todo!()
            }
            Decl::Use(u) => Decl::Use(u.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Decl::Var(v) => Decl::Var(v.with_trailing_trivia(trivia)),
            Decl::Fun(f) => {
                Decl::Fun(f.with_trailing_trivia(trivia))
            }
            Decl::Struct(_) => {
                todo!()
            }
            Decl::ExternC(_) => {
                todo!()
            }
            Decl::Enum { .. } => {
                todo!()
            }
            Decl::Protocol { .. } => {
                todo!()
            }
            Decl::Extension { .. } => {
                todo!()
            }
            Decl::Use(u) => Decl::Use(u.with_trailing_trivia(trivia)),
        }
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
    Method(FunSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StoredPropertySyntax {
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InitializerSyntax {
    pub init_keyword: TokenSyntax,
    pub args: ArgDefListSyntax,
    pub body: FunBody,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DeinitializerSyntax {
    pub deinit_keyword: TokenSyntax,
    pub body: FunBody,
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
