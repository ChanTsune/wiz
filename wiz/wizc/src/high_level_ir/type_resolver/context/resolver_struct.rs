use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StructKind {
    Struct,
    Protocol,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ResolverStruct {
    self_: TypedType, // TODO: remove this field
    pub(crate) namespace: Vec<String>,
    pub(crate) kind: StructKind,
    pub(crate) stored_properties: HashMap<String, TypedType>,
    pub(crate) computed_properties: HashMap<String, TypedType>,
    pub(crate) member_functions: HashMap<String, TypedType>,
    pub(crate) static_functions: HashMap<String, TypedType>,
    pub(crate) conformed_protocols: HashSet<String>,
    pub(crate) type_parameters: Option<HashMap<String, ResolverTypeParam>>,
}

impl ResolverStruct {
    pub fn new(self_: TypedType, kind: StructKind) -> Self {
        Self {
            namespace: self_.package().into_resolved().names,
            self_,
            kind,
            stored_properties: Default::default(),
            computed_properties: Default::default(),
            member_functions: Default::default(),
            static_functions: Default::default(),
            conformed_protocols: Default::default(),
            type_parameters: None, // TODO: fill type params
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

    pub(crate) fn self_type(&self) -> TypedType {
        self.self_.clone()
    }

    pub fn is_generic(&self) -> bool {
        self.type_parameters.is_some()
    }
}
