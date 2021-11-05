use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IfExprSyntax {
    pub if_keyword: TokenSyntax,
    pub condition: Box<Expr>,
    pub body: BlockSyntax,
    pub else_body: Option<ElseSyntax>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ElseSyntax {
    pub else_keyword: TokenSyntax,
    pub body: BlockSyntax,
}
