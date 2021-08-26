use std::collections::{HashMap, HashSet};
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverStruct {
    stored_properties: HashMap<String, TypedType>,
    // initializers: Vec<>,
    computed_properties: HashMap<String, TypedType>,
    member_functions: HashMap<String, TypedType>,
    static_functions: HashMap<String, TypedType>,
    conformed_protocols: HashSet<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct NameSpace {
    pub(crate) children: HashMap<String, NameSpace>,
    pub(crate) types: HashMap<String, ResolverStruct>,
    pub(crate) structs: HashMap<String, TypedType>,
    pub(crate) functions: HashMap<String, TypedType>,
    pub(crate) values: HashMap<String, TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverSubscript {
    target: TypedType,
    indexes: Vec<TypedType>,
    return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverBinary {
    right: TypedType,
    left: TypedType,
    return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverUnary {
    value: TypedType,
    return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverContext {
    name_space: NameSpace,
    binary_operators: HashMap<BinaryOperator, Vec<ResolverBinary>>,
    subscripts: Vec<ResolverSubscript>,
    current_namespace: Vec<String>,
}

impl ResolverStruct {
    pub fn new() -> Self {
        Self {
            stored_properties: Default::default(),
            computed_properties: Default::default(),
            member_functions: Default::default(),
            static_functions: Default::default(),
            conformed_protocols: Default::default(),
            type_params: None,
        }
    }

    pub fn is_generic(&self) -> bool {
        self.type_params != None
    }
}

impl NameSpace {
    fn new() -> Self {
        Self {
            children: Default::default(),
            types: Default::default(),
            structs: Default::default(),
            functions: Default::default(),
            values: Default::default(),
        }
    }

    fn get_child_mut(&mut self, mut ns: Vec<String>) -> Option<&mut NameSpace> {
        if ns.is_empty() {
            Some(self)
        } else {
            let n = ns.remove(0);
            let m = self.children.get_mut(&*n)?;
            m.get_child_mut(ns)
        }
    }

    fn set_child(&mut self, mut ns: Vec<String>) {
        if !ns.is_empty() {
            let n = &ns.remove(0);
            if !self.children.contains_key(n) {
                self.children.insert(n.clone(), NameSpace::new());
            };
            self.children.get_mut(n).unwrap().set_child(ns);
        }
    }
}

impl ResolverContext {
    pub(crate) fn new() -> Self {
        Self {
            name_space: NameSpace::new(),
            binary_operators: Default::default(),
            subscripts: vec![],
            current_namespace: vec![],
        }
    }

    pub fn push_name_space(&mut self, name: String) {
        self.current_namespace.push(name);
    }

    pub fn pop_name_space(&mut self) {
        self.current_namespace.pop();
    }

    pub fn get_current_namespace_mut(&mut self) -> Option<&mut NameSpace> {
        self.name_space
            .get_child_mut(self.current_namespace.clone())
    }
}
