use crate::syntax::expression::Expr;
use crate::syntax::list::{ElementSyntax, ListSyntax};
use crate::syntax::statement::Stmt;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallExprSyntax {
    pub target: Box<Expr>,
    pub args: Option<CallArgListSyntax>,
    pub tailing_lambda: Option<LambdaSyntax>,
}

pub type CallArgListSyntax = ListSyntax<CallArg>;

impl CallArgListSyntax {
    pub(crate) fn new() -> Self {
        Self {
            open: TokenSyntax::from("("),
            elements: vec![],
            close: TokenSyntax::from(")")
        }
    }
}

impl Default for CallArgListSyntax {
    fn default() -> Self {
        Self::new()
    }
}

pub type CallArgElementSyntax = ElementSyntax<CallArg>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub label: Option<TokenSyntax>,
    pub asterisk: Option<TokenSyntax>,
    pub arg: Box<Expr>,
}

impl Syntax for CallArg {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.label {
            None => {
                match self.asterisk {
                    None => {
                        Self {
                            label: None,
                            asterisk: None,
                            arg: Box::new(self.arg.with_trailing_trivia(trivia))
                        }
                    }
                    Some(asterisk) => {
                        Self {
                            label: None,
                            asterisk: Some(asterisk.with_leading_trivia(trivia)),
                            arg: Box::new(*self.arg)
                        }
                    }
                }
            }
            Some(label) => {
                Self {
                    label: Some(label.with_leading_trivia(trivia)),
                    asterisk: self.asterisk,
                    arg: self.arg
                }
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            label: self.label,
            asterisk: self.asterisk,
            arg: Box::new(self.arg.with_trailing_trivia(trivia))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LambdaSyntax {
    pub stmts: Vec<Stmt>,
}
