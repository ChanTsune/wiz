use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedAnnotations {
    annotations: Vec<String>,
}

impl TypedAnnotations {
    pub(crate) fn has_annotate<T: ToString>(&self, a: T) -> bool {
        self.annotations.contains(&a.to_string())
    }
}

impl<T: ToString> From<Vec<T>> for TypedAnnotations {
    fn from(annotations: Vec<T>) -> Self {
        Self { annotations: annotations.iter().map(T::to_string).collect() }
    }
}
