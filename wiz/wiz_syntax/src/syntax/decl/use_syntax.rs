use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UseSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub use_keyword: TokenSyntax,
    pub package_name: PackageName,
    pub alias: Option<AliasSyntax>,
}

impl Annotatable for UseSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

impl Syntax for UseSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        if self.annotations.is_some() {
            Self {
                annotations: None,
                use_keyword: self.use_keyword,
                package_name: self.package_name,
                alias: None,
            }
        } else {
            Self {
                annotations: None,
                use_keyword: self.use_keyword.with_leading_trivia(trivia),
                package_name: self.package_name,
                alias: None,
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            annotations: self.annotations,
            use_keyword: self.use_keyword,
            package_name: self.package_name,
            alias: self.alias,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PackageName {
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AliasSyntax {
    pub as_keyword: TokenSyntax,
    pub name: TokenSyntax,
}
