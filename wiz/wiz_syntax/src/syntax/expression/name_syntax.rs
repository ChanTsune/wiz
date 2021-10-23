use crate::syntax::name_space::NameSpaceSyntax;
use crate::syntax::node::SyntaxNode;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameExprSyntax {
    pub name_space: NameSpaceSyntax,
    pub name: TokenSyntax,
}

impl Syntax for NameExprSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            name_space: self.name_space.with_leading_trivia(trivia),
            name: self.name,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            name_space: self.name_space,
            name: self.name.with_trailing_trivia(trivia),
        }
    }
}

impl SyntaxNode for NameExprSyntax {}
