use crate::syntax::declaration::TypeAnnotationSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_annotation: Option<TypeAnnotationSyntax>,
    pub equal: TokenSyntax,
    pub value: Expr,
}

impl Syntax for VarSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_annotation: self.type_annotation,
            equal: self.equal,
            value: self.value,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword,
            name: self.name,
            type_annotation: self.type_annotation,
            equal: self.equal,
            value: self.value.with_trailing_trivia(trivia),
        }
    }
}
