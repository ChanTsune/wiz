use crate::ast::Page;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpellBook {
    name: String,
    pages: Vec<Page>,
}
