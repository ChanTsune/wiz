use std::collections::{HashMap, HashSet};
use wiz_hir::typed_type::{Package, TypedNamedValueType, TypedPackage, TypedType, TypedValueType};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArenaTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ArenaTypeParam>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StructKind {
    Struct,
    Protocol,
    TypeParameter,
}

impl StructKind {
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct)
    }

    pub fn is_protocol(&self) -> bool {
        matches!(self, Self::Protocol)
    }

    pub fn is_type_parameter(&self) -> bool {
        matches!(self, Self::TypeParameter)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArenaStruct {
    pub namespace: Vec<String>,
    name: String,
    pub kind: StructKind,
    pub stored_properties: HashMap<String, TypedType>,
    pub computed_properties: HashMap<String, TypedType>,
    pub member_functions: HashMap<String, TypedType>,
    pub conformed_protocols: HashSet<String>,
    pub type_parameters: Option<HashMap<String, ArenaTypeParam>>,
}

impl ArenaStruct {
    pub fn new(name: &str, namespace: &[String], kind: StructKind) -> Self {
        Self {
            namespace: namespace.to_vec(),
            name: name.to_owned(),
            kind,
            stored_properties: Default::default(),
            computed_properties: Default::default(),
            member_functions: Default::default(),
            conformed_protocols: Default::default(),
            type_parameters: None, // TODO: fill type params
        }
    }

    pub fn get_instance_member_type(&self, name: &str) -> Option<&TypedType> {
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

    pub fn self_type(&self) -> TypedType {
        TypedType::Value(TypedValueType::Value(TypedNamedValueType {
            package: TypedPackage::Resolved(Package::from(&self.namespace)),
            name: self.name.clone(),
            type_args: None,
        }))
    }

    pub fn is_generic(&self) -> bool {
        self.type_parameters.is_some()
    }

    pub fn is_type_parameter(&self) -> bool {
        self.kind.is_type_parameter()
    }
}
