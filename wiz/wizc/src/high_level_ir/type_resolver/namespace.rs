use crate::high_level_ir::declaration_id::DeclarationId;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Namespace {
    name: String,
    parent: Option<DeclarationId>,
    children: HashMap<String, DeclarationId>,
}

impl Namespace {
    pub fn root() -> Self {
        Self::default()
    }

    pub fn new(name: &str, parent: DeclarationId) -> Self {
        Self {
            name: name.to_string(),
            parent: Some(parent),
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, name: &str, id: DeclarationId) {
        self.children.insert(name.to_string(), id);
    }

    pub fn get_child(&self, name: &str) -> Option<DeclarationId> {
        self.children.get(name).copied()
    }

    pub fn parent(&self) -> Option<DeclarationId> {
        self.parent
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
