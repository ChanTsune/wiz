use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub(crate) names: Vec<String>
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedType {
    pub(crate) package: Package,
    pub(crate) name: String
}

impl TypedType {
    pub fn noting() -> TypedType {
        TypedType { package: Package { names: vec![] }, name: "Noting".to_string() }
    }
}
