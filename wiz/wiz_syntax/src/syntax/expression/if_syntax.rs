use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IfExprSyntax {
    pub if_keyword: TokenSyntax,
    pub condition: Box<Expr>,
    pub body: BlockSyntax,
    pub else_body: Option<ElseSyntax>,
}

impl Syntax for IfExprSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            if_keyword: self.if_keyword.with_leading_trivia(trivia),
            condition: self.condition,
            body: self.body,
            else_body: self.else_body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.else_body {
            None => Self {
                if_keyword: self.if_keyword,
                condition: self.condition,
                body: self.body.with_trailing_trivia(trivia),
                else_body: None
            },
            Some(else_body) => Self {
                if_keyword: self.if_keyword,
                condition: self.condition,
                body: self.body,
                else_body: Some(else_body.with_trailing_trivia(trivia))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ElseSyntax {
    pub else_keyword: TokenSyntax,
    pub body: BlockSyntax,
}


impl Syntax for ElseSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            else_keyword: self.else_keyword.with_leading_trivia(trivia),
            body: self.body
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            else_keyword: self.else_keyword,
            body: self.body.with_trailing_trivia(trivia)
        }
    }
}