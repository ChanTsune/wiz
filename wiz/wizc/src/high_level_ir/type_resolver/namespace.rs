use crate::high_level_ir::declaration_id::DeclarationId;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Namespace {
    parent: Option<DeclarationId>,
}

impl Namespace {
    pub fn root() -> Self {
        Self::default()
    }

    pub fn new(parent: DeclarationId) -> Self {
        Self {
            parent: Some(parent),
        }
    }

    pub fn parent(&self) -> Option<DeclarationId> {
        self.parent
    }
}
