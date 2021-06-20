use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
enum TypedDecl {
    Var,
    Fun,
    Struct,
    Class,
    Enum,
    Protocol,
    Extension
}
