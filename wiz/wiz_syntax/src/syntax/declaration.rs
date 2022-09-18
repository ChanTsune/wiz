use crate::syntax::annotation::AnnotationsSyntax;
pub use crate::syntax::declaration::extension_syntax::{ExtensionSyntax, ProtocolConformSyntax};
use crate::syntax::declaration::fun_syntax::FunSyntax;
pub use crate::syntax::declaration::properties_syntax::{
    DeinitializerSyntax, StoredPropertySyntax, StructBodySyntax, StructPropertySyntax,
};
pub use crate::syntax::declaration::struct_syntax::StructSyntax;
pub use crate::syntax::declaration::use_syntax::{
    AliasSyntax, PackageName, PackageNameElement, UseSyntax,
};
pub use crate::syntax::declaration::var_syntax::VarSyntax;
use crate::syntax::file::FileSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;
use crate::syntax::Syntax;

mod extension_syntax;
pub mod fun_syntax;
mod properties_syntax;
mod struct_syntax;
mod use_syntax;
mod var_syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DeclarationSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub kind: DeclKind,
}

impl Syntax for DeclarationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.annotations {
            None => Self {
                annotations: None,
                kind: self.kind.with_leading_trivia(trivia),
            },
            Some(annotations) => Self {
                annotations: Some(annotations.with_leading_trivia(trivia)),
                kind: self.kind,
            },
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            annotations: self.annotations,
            kind: self.kind.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DeclKind {
    Var(VarSyntax),
    Fun(FunSyntax),
    Struct(StructSyntax),
    ExternC(ExternCSyntax),
    Enum {
        // TODO
    },
    Module(ModuleSyntax),
    Extension(ExtensionSyntax),
    Use(UseSyntax),
}

impl Syntax for DeclKind {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            DeclKind::Var(v) => DeclKind::Var(v.with_leading_trivia(trivia)),
            DeclKind::Fun(f) => DeclKind::Fun(f.with_leading_trivia(trivia)),
            DeclKind::Struct(s) => DeclKind::Struct(s.with_leading_trivia(trivia)),
            DeclKind::ExternC(_) => {
                todo!()
            }
            DeclKind::Enum { .. } => {
                todo!()
            }
            DeclKind::Extension(e) => DeclKind::Extension(e.with_leading_trivia(trivia)),
            DeclKind::Use(u) => DeclKind::Use(u.with_leading_trivia(trivia)),
            DeclKind::Module(m) => DeclKind::Module(m.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            DeclKind::Var(v) => DeclKind::Var(v.with_trailing_trivia(trivia)),
            DeclKind::Fun(f) => DeclKind::Fun(f.with_trailing_trivia(trivia)),
            DeclKind::Struct(s) => DeclKind::Struct(s.with_trailing_trivia(trivia)),
            DeclKind::ExternC(_) => {
                todo!()
            }
            DeclKind::Enum { .. } => {
                todo!()
            }
            DeclKind::Extension(e) => DeclKind::Extension(e.with_trailing_trivia(trivia)),
            DeclKind::Use(u) => DeclKind::Use(u.with_trailing_trivia(trivia)),
            DeclKind::Module(m) => DeclKind::Module(m.with_trailing_trivia(trivia)),
        }
    }
}

pub type ModuleSyntax = (String, Option<FileSyntax>);

impl Syntax for ModuleSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        self
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExternCSyntax {
    pub extern_keyword: TokenSyntax,
    pub left_brace: TokenSyntax,
    pub declarations: Vec<DeclKind>,
    pub right_brace: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeAnnotationSyntax {
    pub colon: TokenSyntax,
    pub type_: TypeName,
}

impl Syntax for TypeAnnotationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            colon: self.colon.with_leading_trivia(trivia),
            type_: self.type_,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            colon: self.colon,
            type_: self.type_.with_trailing_trivia(trivia),
        }
    }
}
