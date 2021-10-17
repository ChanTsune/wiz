use crate::syntax::trivia::Trivia;

pub mod annotation;
pub mod block;
pub mod decl;
pub mod expr;
pub mod file;
pub mod fun;
pub mod literal;
pub mod name_space;
pub mod node;
pub mod stmt;
pub mod token;
pub mod trivia;
pub mod type_name;

pub trait Syntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self;
    fn with_trailing_trivia(self, trivia: Trivia) -> Self;
}
