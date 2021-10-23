use std::fmt::Debug;
use crate::syntax::trivia::Trivia;

pub mod annotation;
pub mod block;
pub mod decl;
pub mod expression;
pub mod file;
pub mod literal;
pub mod modifier;
pub mod name_space;
pub mod stmt;
pub mod token;
pub mod trivia;
pub mod type_name;

pub trait Syntax: Debug + Eq + PartialEq + Clone {
    fn with_leading_trivia(self, trivia: Trivia) -> Self;
    fn with_trailing_trivia(self, trivia: Trivia) -> Self;
}
