use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Package {
    names: Vec<String>
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedType {
    package: Package,
    name: String
}
