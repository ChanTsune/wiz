use crate::syntax::expression::Expr;
use crate::syntax::list::{ElementSyntax, ListSyntax};

pub type ArraySyntax = ListSyntax<Expr>;
pub type ArrayElementSyntax = ElementSyntax<Expr>;
