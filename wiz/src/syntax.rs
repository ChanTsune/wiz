use crate::syntax::trivia::Trivia;

pub mod block;
pub mod decl;
pub mod expr;
pub mod file;
pub mod fun;
pub mod literal;
pub mod node;
pub mod stmt;
pub mod token;
pub mod trivia;
pub mod type_name;

trait Syntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self;
    fn with_trailing_trivia(self, trivia: Trivia) -> Self;
}
