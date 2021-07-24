use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLType {
    pub(crate) name: String,
}
