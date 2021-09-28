use crate::syntax::node::SyntaxNode;
use crate::syntax::token::TokenSyntax;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LiteralSyntax {
    Integer(TokenSyntax),
    FloatingPoint(TokenSyntax),
    String {
        open_quote: TokenSyntax,
        value: String,
        close_quote: TokenSyntax,
    },
    Boolean(TokenSyntax),
    Null,
}

impl SyntaxNode for LiteralSyntax {}
