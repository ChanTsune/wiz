
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedAnnotations {
    annotations: Vec<String>
}

impl TypedAnnotations {
    pub(crate) fn new() -> Self {
        Self { annotations: vec![] }
    }
}

impl From<Vec<String>> for TypedAnnotations{
    fn from(annotations: Vec<String>) -> Self {
        Self {
            annotations
        }
    }
}