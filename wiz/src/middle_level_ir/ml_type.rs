use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLType {
    pub(crate) name: String,
}
