use crate::expr::{MLExpr, MLName};
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::MLValueType;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLCall {
    pub target: MLName,
    pub args: Vec<MLCallArg>,
    pub type_: MLValueType,
}

impl MLNode for MLCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('(')?;
        for (c, arg) in self.args.iter().enumerate() {
            arg.fmt(f)?;
            let s = self.args.len() - 1;
            if s != c {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLCallArg {
    pub arg: MLExpr,
}

impl MLNode for MLCallArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.arg.fmt(f)
    }
}
