use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemberSyntax {
    pub target: Box<Expr>,
    pub navigation_operator: TokenSyntax,
    pub name: TokenSyntax,
}

impl Syntax for MemberSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: Box::new(self.target.with_leading_trivia(trivia)),
            navigation_operator: self.navigation_operator,
            name: self.name
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            navigation_operator: self.navigation_operator,
            name: self.name.with_trailing_trivia(trivia)
        }
    }
}