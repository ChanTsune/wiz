mod assignment_syntax;
mod for_loop_syntax;

use crate::syntax::block::BlockSyntax;
use crate::syntax::declaration::Decl;
use crate::syntax::expression::Expr;
pub use crate::syntax::statement::assignment_syntax::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax,
};
pub use crate::syntax::statement::for_loop_syntax::ForLoopSyntax;
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
        match self {
            Stmt::Decl(d) => Stmt::Decl(d.with_leading_trivia(trivia)),
            Stmt::Expr(e) => Stmt::Expr(e.with_leading_trivia(trivia)),
            Stmt::Assignment(a) => Stmt::Assignment(a),
            Stmt::Loop(_) => {
                todo!()
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Stmt::Decl(d) => Stmt::Decl(d.with_trailing_trivia(trivia)),
            Stmt::Expr(e) => Stmt::Expr(e.with_trailing_trivia(trivia)),
            Stmt::Assignment(a) => Stmt::Assignment(a),
            Stmt::Loop(_) => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While(WhileLoopSyntax),
    For(ForLoopSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub condition: Expr,
    pub block: BlockSyntax,
}
