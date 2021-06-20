use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
enum TypedExpr {
    Name,
    Literal,
    BinOp,
    UnaryOp,
    Subscript,
    List,
    Tuple,
    Dict,
    StringBuilder,
    Call,
    If,
    When,
    Lambda,
    Return,
    TypeCast
}
