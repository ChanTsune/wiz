use crate::syntax::block::BlockSyntax;
use crate::syntax::decl::Decl;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
    Assignment(AssignmentStmt),
    Loop(LoopStmt),
}

impl Syntax for Stmt {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AssignmentStmt {
    Assignment(AssignmentSyntax),
    AssignmentAndOperator(AssignmentAndOperatorSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub target: Expr,
    pub operator: TokenSyntax,
    pub value: Expr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentAndOperatorSyntax {
    pub target: Expr,
    pub operator: TokenSyntax,
    pub value: Expr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While(WhileLoopSyntax),
    For {
        values: Vec<String>,
        iterator: Expr,
        block: BlockSyntax,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub condition: Expr,
    pub block: BlockSyntax,
}
