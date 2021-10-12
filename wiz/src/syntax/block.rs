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

impl SyntaxNode for BlockSyntax {}
