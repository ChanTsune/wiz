use crate::typed_type::Package;
use serde::{Deserialize, Serialize};
use wiz_data_structure::annotation::Annotations;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedUse {
    pub annotations: Annotations,
    pub package: Package,
    pub alias: Option<String>,
}

impl<T: ToString, const N: usize> From<&[T; N]> for TypedUse {
    fn from(vec: &[T; N]) -> Self {
        Self {
            annotations: Default::default(),
            package: Package::from(vec),
            alias: None,
        }
    }
}
