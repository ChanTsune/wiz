use crate::syntax::node::SyntaxNode;
use crate::syntax::stmt::Stmt;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BlockSyntax {
    pub(crate) body: Vec<Stmt>,
}

impl SyntaxNode for BlockSyntax {}
