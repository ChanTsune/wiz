use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    pub(crate) uses: Vec<Use>,
    pub(crate) pages: HashMap<String, Page>,
}

impl Page {
    pub(crate) fn empty() -> Self {
        Self {
            uses: Vec::new(),
            pages: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Use {
    namespace: Vec<String>,
    name: String,
    alias: Option<String>,
}

impl Use {
    pub(crate) fn new(namespace: Vec<String>, name: String, alias: Option<String>) -> Self {
        Self {
            namespace,
            name,
            alias,
        }
    }
}