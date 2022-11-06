use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Annotations {
    annotations: Vec<String>,
}

impl Annotations {
    pub fn has_annotate<T: ToString>(&self, a: T) -> bool {
        self.annotations.contains(&a.to_string())
    }
}

impl<T: ToString> From<Vec<T>> for Annotations {
    fn from(annotations: Vec<T>) -> Self {
        Self {
            annotations: annotations.iter().map(T::to_string).collect(),
        }
    }
}
