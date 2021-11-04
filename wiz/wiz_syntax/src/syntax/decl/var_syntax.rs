use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_: Option<TypeName>,
    pub value: Expr,
}

impl Annotatable for VarSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

impl Syntax for VarSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.annotations {
            None => {
                Self {
                    annotations: None,
                    mutability_keyword: self.mutability_keyword.with_leading_trivia(trivia),
                    name: self.name,
                    type_: self.type_,
                    value: self.value
                }
            }
            Some(annotations) => {
                Self {
                    annotations: Some(annotations.with_leading_trivia(trivia)),
                    mutability_keyword: self.mutability_keyword,
                    name: self.name,
                    type_: self.type_,
                    value: self.value
                }
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            annotations: self.annotations,
            mutability_keyword: self.mutability_keyword,
            name: self.name,
            type_: self.type_,
            value: self.value.with_trailing_trivia(trivia)
        }
    }
}
