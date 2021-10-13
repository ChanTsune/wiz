use crate::syntax::node::SyntaxNode;
use crate::syntax::stmt::Stmt;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockSyntax {
    pub(crate) open: TokenSyntax,
    pub(crate) body: Vec<Stmt>,
    pub(crate) close: TokenSyntax,
}

impl Syntax for BlockSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open.with_leading_trivia(trivia),
            body: self.body,
            close: self.close
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open,
            body: self.body,
            close: self.close.with_trailing_trivia(trivia)
        }
    }
}

impl SyntaxNode for BlockSyntax {}
