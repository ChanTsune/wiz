use crate::typed_type::Package;
use serde::{Deserialize, Serialize};
use wiz_data_structure::annotation::TypedAnnotations;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedUse {
    pub annotations: TypedAnnotations,
    pub package: Package,
    pub alias: Option<String>,
}

impl<T: ToString> From<Vec<T>> for TypedUse {
    fn from(vec: Vec<T>) -> Self {
        Self {
            annotations: Default::default(),
            package: Package::from(&vec),
            alias: None,
        }
    }
}
