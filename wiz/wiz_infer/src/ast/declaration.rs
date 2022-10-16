use crate::ast::TopLevel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    uses: Vec<Use>,
    items: Vec<TopLevel>,
}

impl Page {
    pub(crate) fn empty() -> Self {
        Self {
            uses: Vec::new(),
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Use {
    names: Vec<String>,
    alias: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevelVar {
    name: String,
}
