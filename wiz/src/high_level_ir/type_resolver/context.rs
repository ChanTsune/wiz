use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverStruct {
    pub(crate) stored_properties: HashMap<String, TypedType>,
    // pub(crate) initializers: Vec<>,
    pub(crate) computed_properties: HashMap<String, TypedType>,
    pub(crate) member_functions: HashMap<String, TypedType>,
    pub(crate) static_functions: HashMap<String, TypedType>,
    pub(crate) conformed_protocols: HashSet<String>,
    pub(crate) type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct NameSpace {
    pub(crate) children: HashMap<String, NameSpace>,
    pub(crate) types: HashMap<String, ResolverStruct>,
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
    pub(crate) current_namespace: Vec<String>,
    current_type: Option<TypedType>,
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
        let mut ns = NameSpace::new();
        for t in TypedType::builtin_types() {
            match &t {
                TypedType::Value(v) => {
                    ns.types.insert(v.name.clone(), ResolverStruct::new());
                }
                TypedType::Function(_) => {}
            };
        }
        Self {
            name_space: ns,
            binary_operators: Default::default(),
            subscripts: vec![],
            current_namespace: vec![],
            current_type: None,
        }
    }

    pub fn push_name_space(&mut self, name: String) {
        self.current_namespace.push(name);
        self.name_space.set_child(self.current_namespace.clone());
    }

    pub fn pop_name_space(&mut self) {
        self.current_namespace.pop();
    }

    pub fn get_current_namespace_mut(&mut self) -> Option<&mut NameSpace> {
        println!("NS => {:?}", self.current_namespace);
        self.name_space
            .get_child_mut(self.current_namespace.clone())
    }

    pub fn get_namespace_mut(&mut self, ns: Vec<String>) -> Option<&mut NameSpace> {
        self.name_space.get_child_mut(ns)
    }

    pub fn get_current_type(&self) -> Option<TypedType> {
        self.current_type.clone()
    }

    pub fn set_current_type(&mut self, t: TypedType) {
        self.current_type = Some(t)
    }

    pub fn clear_current_type(&mut self) {
        self.current_type = None
    }

    pub fn resolve_member_type(&mut self, t: TypedType, name: String) -> Option<TypedType> {
        match t {
            TypedType::Value(v) => {
                let ns = self.get_namespace_mut(v.package.names)?;
                println!("ns => {:?}", ns);
                match ns.types.get(&v.name) {
                    Some(rs) => rs.stored_properties.get(&name).map(|it| it.clone()),
                    None => None,
                }
            }
            TypedType::Function(_) => None,
        }
    }

    pub fn resolve_name_type(&mut self, name: String) -> Option<TypedType> {
        let mut cns = self.current_namespace.clone();
        loop {
            let ns = self.get_namespace_mut(cns.clone())?;
            if let Some(t) = ns.values.get(&name) {
                return Some(t.clone());
            }
            if cns.is_empty() {
                break;
            }
            cns.pop();
        }
        None
    }
}

mod test {
    use crate::high_level_ir::type_resolver::context::NameSpace;
    use crate::high_level_ir::typed_type::TypedType;

    #[test]
    fn test_name_space() {
        let mut name_space = NameSpace::new();
        name_space
            .values
            .insert(String::from("Int64"), TypedType::int64());
        name_space.set_child(vec![String::from("builtin")]);
        assert_eq!(
            name_space.get_child_mut(vec![String::from("builtin")]),
            Some(&mut NameSpace::new())
        )
    }
}
