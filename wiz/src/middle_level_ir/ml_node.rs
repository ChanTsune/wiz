use crate::middle_level_ir::format::Formatter;
use std::fmt;

pub trait MLNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
}
