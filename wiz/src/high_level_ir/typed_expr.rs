use std::fmt;
use crate::high_level_ir::typed_type::TypedType;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedExpr {
    Name {
        name: String,
        type_: TypedType
    },
    Literal(TypedLiteral),
    BinOp {
        left: Box<TypedExpr>,
        kind: String,
        right: Box<TypedExpr>,
        type_: TypedType
    },
    UnaryOp {
        target: Box<TypedExpr>,
        prefix: bool,
        kind: String,
        type_: TypedType
    },
    Subscript,
    List,
    Tuple,
    Dict,
    StringBuilder,
    Call {
        target: Box<TypedExpr>,
        args: Vec<TypedCallArg>,
    },
    If,
    When,
    Lambda,
    Return,
    TypeCast
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedLiteral {
    Integer {
        value: String,
        type_: TypedType
    },
    FloatingPoint {
        value: String,
        type_: TypedType
    },
    String {
        value: String,
        type_: TypedType
    },
    Boolean {
        value: String,
        type_: TypedType
    },
    NullLiteral {
        type_: TypedType
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedCallArg {
    label: Option<String>,
    arg: Box<TypedExpr>,
    is_vararg: bool,
}
