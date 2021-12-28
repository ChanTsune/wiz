use crate::expr::MLExpr;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::MLValueType;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLArray {
    pub elements: Vec<MLExpr>,
    pub type_: MLValueType,
}

impl MLNode for MLArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (i, element) in self.elements.iter().enumerate() {
            element.fmt(f)?;
            let last_index = self.elements.len() - 1;
            if i != last_index {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}
