use std::fmt::Debug;

pub trait SyntaxNode: Debug + Eq + PartialEq + Clone {}
