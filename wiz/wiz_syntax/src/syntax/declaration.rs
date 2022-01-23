use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
pub use crate::syntax::declaration::extension_syntax::{ExtensionSyntax, ProtocolConformSyntax};
use crate::syntax::declaration::fun_syntax::FunSyntax;
pub use crate::syntax::declaration::struct_syntax::{
    DeinitializerSyntax, InitializerSyntax, StoredPropertySyntax, StructPropertySyntax,
    StructSyntax,
};
pub use crate::syntax::declaration::use_syntax::{
    AliasSyntax, PackageName, PackageNameElement, UseSyntax,
};
pub use crate::syntax::declaration::var_syntax::VarSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

mod extension_syntax;
pub mod fun_syntax;
mod struct_syntax;
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
    Extension(ExtensionSyntax),
    Use(UseSyntax),
}

impl Annotatable for Decl {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self {
        match self {
            Decl::Var(v) => Decl::Var(v.with_annotation(a)),
            Decl::Fun(f) => Decl::Fun(f.with_annotation(a)),
            Decl::Struct(s) => Decl::Struct(s.with_annotation(a)),
            Decl::ExternC(e) => Decl::ExternC(e.with_annotation(a)),
            Decl::Enum { .. } => Decl::Enum {},
            Decl::Extension(e) => Decl::Extension(e.with_annotation(a)),
            Decl::Use(u) => Decl::Use(u.with_annotation(a)),
        }
    }
}

impl Syntax for Decl {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            Decl::Var(v) => Decl::Var(v.with_leading_trivia(trivia)),
            Decl::Fun(f) => Decl::Fun(f.with_leading_trivia(trivia)),
            Decl::Struct(s) => Decl::Struct(s.with_leading_trivia(trivia)),
            Decl::ExternC(_) => {
                todo!()
            }
            Decl::Enum { .. } => {
                todo!()
            }
            Decl::Extension(e) => Decl::Extension(e.with_leading_trivia(trivia)),
            Decl::Use(u) => Decl::Use(u.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Decl::Var(v) => Decl::Var(v.with_trailing_trivia(trivia)),
            Decl::Fun(f) => Decl::Fun(f.with_trailing_trivia(trivia)),
            Decl::Struct(s) => Decl::Struct(s.with_trailing_trivia(trivia)),
            Decl::ExternC(_) => {
                todo!()
            }
            Decl::Enum { .. } => {
                todo!()
            }
            Decl::Extension(e) => Decl::Extension(e.with_trailing_trivia(trivia)),
            Decl::Use(u) => Decl::Use(u.with_trailing_trivia(trivia)),
        }
    }
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
