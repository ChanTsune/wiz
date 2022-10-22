use std::collections::HashMap;
use wiz_syntax::syntax::declaration::{ExtensionSyntax, FunSyntax, StructSyntax, VarSyntax};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    pub(crate) uses: Vec<Use>,
    pub(crate) var_defs: Vec<VarSyntax>,
    pub(crate) function_defs: Vec<FunSyntax>,
    pub(crate) struct_defs: Vec<StructSyntax>,
    pub(crate) extension_defs: Vec<ExtensionSyntax>,
    pub(crate) pages: HashMap<String, Page>,
}

impl Page {
    pub(crate) fn empty() -> Self {
        Self {
            uses: Vec::new(),
            function_defs: Vec::new(),
            var_defs: Vec::new(),
            struct_defs: Vec::new(),
            extension_defs: Vec::new(),
            pages: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Use {
    pub(crate) namespace: Vec<String>,
    pub(crate) name: String,
    pub(crate) alias: Option<String>,
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
