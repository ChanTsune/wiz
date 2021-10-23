use crate::syntax::node::SyntaxNode;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Syntax for LiteralSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            LiteralSyntax::Integer(i) => LiteralSyntax::Integer(i.with_leading_trivia(trivia)),
            LiteralSyntax::FloatingPoint(f) => LiteralSyntax::FloatingPoint(f.with_leading_trivia(trivia)),
            LiteralSyntax::Boolean(b) => LiteralSyntax::Boolean(b.with_leading_trivia(trivia)),
            LiteralSyntax::String { open_quote, value, close_quote } => LiteralSyntax::String {
                open_quote: open_quote.with_leading_trivia(trivia),
                value,
                close_quote
            },
            LiteralSyntax::Null => {todo!()}
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            LiteralSyntax::Integer(i) => LiteralSyntax::Integer(i.with_trailing_trivia(trivia)),
            LiteralSyntax::FloatingPoint(f) => LiteralSyntax::FloatingPoint(f.with_trailing_trivia(trivia)),
            LiteralSyntax::Boolean(b) => LiteralSyntax::Boolean(b.with_trailing_trivia(trivia)),
            LiteralSyntax::String { open_quote, value, close_quote } => LiteralSyntax::String {
                open_quote,
                value,
                close_quote: close_quote.with_trailing_trivia(trivia)
            },
            LiteralSyntax::Null => {todo!()}
        }
    }
}

impl SyntaxNode for LiteralSyntax {}
