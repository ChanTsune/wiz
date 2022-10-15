use crate::ast::TopLevel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    uses: Vec<Use>,
    items: Vec<TopLevel>,
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
