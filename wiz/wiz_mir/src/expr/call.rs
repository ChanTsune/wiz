use std::fmt;
use std::fmt::Write;
use crate::expr::MLExpr;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::{MLType, MLValueType};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLCall {
    pub target: Box<MLExpr>,
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

impl MLCallArg {
    pub fn type_(&self) -> MLType {
        self.arg.type_()
    }
}

impl MLNode for MLCallArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.arg.fmt(f)
    }
}
