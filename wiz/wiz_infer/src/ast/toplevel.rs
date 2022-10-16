#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevel {
    pub(crate) name: String,
    pub(crate) kind: TopLevelKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TopLevelKind {
    Var,
    Function,
    Type,
}
