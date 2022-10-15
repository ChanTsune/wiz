use crate::ast::Page;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevel {
    name: String,
    items: TopLevelKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TopLevelKind {
    Namespace(Page),
    Var,
    Function,
    Type,
}
