use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::MLValueType;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLLiteral {
    Integer { value: String, type_: MLValueType },
    FloatingPoint { value: String, type_: MLValueType },
    String { value: String, type_: MLValueType },
    Boolean { value: String, type_: MLValueType },
    Null { type_: MLValueType },
    Struct { type_: MLValueType },
}

impl MLNode for MLLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLLiteral::Integer { value, type_: _ } => f.write_str(value),
            MLLiteral::FloatingPoint { value, type_: _ } => f.write_str(value),
            MLLiteral::String { value, type_: _ } => {
                f.write_char('"')?;
                f.write_str(value)?;
                f.write_char('"')
            }
            MLLiteral::Boolean { value, type_: _ } => f.write_str(value),
            MLLiteral::Null { type_: _ } => fmt::Result::Err(Default::default()),
            MLLiteral::Struct { type_ } => {
                type_.fmt(f)?;
                f.write_str(" { }")
            }
        }
    }
}
