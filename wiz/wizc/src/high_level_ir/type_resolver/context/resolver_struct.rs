use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ResolverStruct {
    pub(crate) self_: TypedType,
    pub(crate) stored_properties: HashMap<String, TypedType>,
    pub(crate) computed_properties: HashMap<String, TypedType>,
    pub(crate) member_functions: HashMap<String, TypedType>,
    pub(crate) static_functions: HashMap<String, TypedType>,
    pub(crate) conformed_protocols: HashSet<String>,
    pub(crate) type_params: Option<HashMap<String, ResolverTypeParam>>,
}

impl ResolverStruct {
    pub fn new(self_: TypedType) -> Self {
        Self {
            self_,
            stored_properties: Default::default(),
            computed_properties: Default::default(),
            member_functions: Default::default(),
            static_functions: Default::default(),
            conformed_protocols: Default::default(),
            type_params: None,
        }
    }

    pub(crate) fn get_instance_member_type(&self, name: &str) -> Option<&TypedType> {
        if let Some(t) = self.stored_properties.get(name) {
            Some(t)
        } else if let Some(t) = self.computed_properties.get(name) {
            Some(t)
        } else if let Some(t) = self.member_functions.get(name) {
            Some(t)
        } else {
            None
        }
    }

    pub fn is_generic(&self) -> bool {
        self.type_params != None
    }
}
