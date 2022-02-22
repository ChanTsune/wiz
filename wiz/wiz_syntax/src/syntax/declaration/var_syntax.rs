use crate::syntax::declaration::TypeAnnotationSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_: Option<TypeAnnotationSyntax>,
    pub value: Expr,
}

impl Syntax for VarSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_: self.type_,
            value: self.value,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword,
            name: self.name,
            type_: self.type_,
            value: self.value.with_trailing_trivia(trivia),
        }
    }
}
