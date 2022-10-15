#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    uses: Vec<Use>,
    items: Vec<TopLevel>,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Use {
    names: Vec<String>,
    alias: Option<String>,
}
