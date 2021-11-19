use crate::expr::{MLBlock, MLExpr};
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::MLValueType;
use std::fmt::{Result, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLIf {
    pub condition: Box<MLExpr>,
    pub body: MLBlock,
    pub else_body: Option<MLBlock>,
    pub type_: MLValueType,
}

impl MLNode for MLIf {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("if (")?;
        self.condition.fmt(f)?;
        f.write_str(") ")?;
        self.body.fmt(f)?;
        match &self.else_body {
            Some(b) => {
                f.write_str(" else ")?;
                b.fmt(f)?;
            }
            None => {}
        };
        Ok(())
    }
}
