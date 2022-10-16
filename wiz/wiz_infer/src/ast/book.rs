use crate::ast::Page;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpellBook {
    pub(crate) name: String,
    pub(crate) page: Page,
}

impl SpellBook {
    pub(crate) fn empty(name: String) -> Self {
        Self {
            name,
            page: Page::empty(),
        }
    }
}
