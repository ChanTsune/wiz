use crate::syntax::expression::Expr;
use crate::syntax::node::SyntaxNode;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryOperationSyntax {
    pub left: Box<Expr>,
    pub operator: TokenSyntax,
    pub right: Box<Expr>,
}

impl Syntax for BinaryOperationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            left: Box::new(self.left.with_leading_trivia(trivia)),
            operator: self.operator,
            right: self.right
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            left: self.left,
            operator: self.operator,
            right: Box::new(self.right.with_trailing_trivia(trivia))
        }
    }
}

impl SyntaxNode for BinaryOperationSyntax {

}