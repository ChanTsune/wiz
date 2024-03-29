use crate::syntax::trivia::Trivia;
use crate::NodeId;
use std::fmt::Debug;
use wiz_span::Location;

pub mod annotation;
pub mod block;
pub mod declaration;
pub mod expression;
mod file;
mod list;
pub mod literal;
pub mod modifier;
pub mod name_space;
pub mod statement;
pub mod token;
pub mod trivia;
pub mod type_name;

pub use file::*;

pub trait Syntax: Debug + Eq + PartialEq + Clone {
    fn with_leading_trivia(self, trivia: Trivia) -> Self;
    fn with_trailing_trivia(self, trivia: Trivia) -> Self;
    fn span(&self) -> Location {
        Location::default()
    }
    fn id(&self) -> NodeId {
        NodeId::DUMMY
    }
}
