mod assignment_syntax;
mod for_loop_syntax;
mod while_loop_syntax;

use crate::syntax::declaration::Decl;
use crate::syntax::expression::Expr;
pub use crate::syntax::statement::assignment_syntax::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax,
};
pub use crate::syntax::statement::for_loop_syntax::ForLoopSyntax;
pub use crate::syntax::statement::while_loop_syntax::WhileLoopSyntax;
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
            Stmt::Assignment(a) => Stmt::Assignment(a.with_leading_trivia(trivia)),
            Stmt::Loop(l) => Stmt::Loop(l.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Stmt::Decl(d) => Stmt::Decl(d.with_trailing_trivia(trivia)),
            Stmt::Expr(e) => Stmt::Expr(e.with_trailing_trivia(trivia)),
            Stmt::Assignment(a) => Stmt::Assignment(a.with_trailing_trivia(trivia)),
            Stmt::Loop(l) => Stmt::Loop(l.with_trailing_trivia(trivia)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While(WhileLoopSyntax),
    For(ForLoopSyntax),
}

impl Syntax for LoopStmt {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            LoopStmt::While(w) => LoopStmt::While(w.with_leading_trivia(trivia)),
            LoopStmt::For(f) => LoopStmt::For(f.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            LoopStmt::While(w) => LoopStmt::While(w.with_trailing_trivia(trivia)),
            LoopStmt::For(f) => LoopStmt::For(f.with_trailing_trivia(trivia)),
        }
    }
}
