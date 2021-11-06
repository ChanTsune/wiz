use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOperationSyntax {
    Prefix(PrefixUnaryOperationSyntax),
    Postfix(PostfixUnaryOperationSyntax),
}

impl Syntax for UnaryOperationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            UnaryOperationSyntax::Prefix(u) => {
                UnaryOperationSyntax::Prefix(u.with_leading_trivia(trivia))
            }
            UnaryOperationSyntax::Postfix(u) => {
                UnaryOperationSyntax::Postfix(u.with_leading_trivia(trivia))
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            UnaryOperationSyntax::Prefix(u) => {
                UnaryOperationSyntax::Prefix(u.with_trailing_trivia(trivia))
            }
            UnaryOperationSyntax::Postfix(u) => {
                UnaryOperationSyntax::Postfix(u.with_trailing_trivia(trivia))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrefixUnaryOperationSyntax {
    pub operator: TokenSyntax,
    pub target: Box<Expr>,
}

impl Syntax for PrefixUnaryOperationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            operator: self.operator.with_leading_trivia(trivia),
            target: self.target,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            operator: self.operator,
            target: Box::new(self.target.with_trailing_trivia(trivia)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PostfixUnaryOperationSyntax {
    pub target: Box<Expr>,
    pub operator: TokenSyntax,
}

impl Syntax for PostfixUnaryOperationSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: Box::new(self.target.with_leading_trivia(trivia)),
            operator: self.operator,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            operator: self.operator.with_trailing_trivia(trivia),
        }
    }
}
