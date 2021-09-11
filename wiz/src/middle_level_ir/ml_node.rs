use std::fmt;
use crate::middle_level_ir::format::Formatter;

pub trait MLNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
}
