use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLExpr {
    Name,
    Literal,
    Call,
    If,
    When,
    Return,
    TypeCast,
}
