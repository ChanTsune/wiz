#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedAnnotations {
    annotations: Vec<String>,
}

impl TypedAnnotations {
    pub(crate) fn has_annotate<T: ToString>(&self, a: T) -> bool {
        self.annotations.contains(&a.to_string())
    }
}

impl TypedAnnotations {
    pub(crate) fn new() -> Self {
        Self {
            annotations: vec![],
        }
    }
}

impl Default for TypedAnnotations {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<String>> for TypedAnnotations {
    fn from(annotations: Vec<String>) -> Self {
        Self { annotations }
    }
}
