use crate::syntax::expression::Expr;
use crate::syntax::list::{ElementSyntax, ListSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SubscriptSyntax {
    pub target: Box<Expr>,
    pub idx_or_keys: SubscriptIndexListSyntax,
}

impl Syntax for SubscriptSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: Box::new(self.target.with_leading_trivia(trivia)),
            idx_or_keys: self.idx_or_keys,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            idx_or_keys: self.idx_or_keys.with_trailing_trivia(trivia),
        }
    }
}

pub type SubscriptIndexListSyntax = ListSyntax<Expr>;

impl SubscriptIndexListSyntax {
    pub(crate) fn new() -> Self {
        Self {
            open: TokenSyntax::from("["),
            elements: vec![],
            close: TokenSyntax::from("]"),
        }
    }
}

pub type SubscriptIndexElementSyntax = ElementSyntax<Expr>;
