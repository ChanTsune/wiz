use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct NameSpace {
    pub name_space: Vec<String>,
    pub values: HashMap<String, EnvValue>,
}

impl NameSpace {
    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn new<T: ToString>(name: Vec<T>) -> Self {
        Self {
            name_space: name.into_iter().map(|i| i.to_string()).collect(),
            values: Default::default(),
        }
    }

    pub(crate) fn get_child<T: ToString>(&self, mut ns: Vec<T>) -> Option<&NameSpace> {
        if ns.is_empty() {
            Some(self)
        } else {
            let n = ns.remove(0).to_string();
            let m = self.values.get(&*n)?;
            match m {
                EnvValue::NameSpace(m) => panic!(),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => None,
            }
        }
    }

    pub(crate) fn set_child<T: ToString>(&mut self, mut ns: Vec<T>) {
        if !ns.is_empty() {
            let n = ns.remove(0).to_string();
            let self_name_space_ref = &self.name_space;
            let entry = self.values.entry(n).or_insert_with_key(|key| {
                let mut name = self_name_space_ref.clone();
                name.push(key.clone());
                panic!()
            });
            match entry {
                EnvValue::NameSpace(n) => panic!(),
                EnvValue::Value(_) => panic!(),
                EnvValue::Type(_) => panic!(),
            };
        }
    }

    pub(crate) fn register_type(&mut self, name: String, s: ResolverStruct) {
        self.values.insert(name, EnvValue::from(s));
    }

    pub(crate) fn get_type(&self, name: &str) -> Option<&ResolverStruct> {
        self.values.get(name).map(|i| match i {
            EnvValue::Type(r) => r,
            _ => panic!(),
        })
    }

    pub(crate) fn get_type_mut(&mut self, name: &str) -> Option<&mut ResolverStruct> {
        self.values.get_mut(name).map(|i| match i {
            EnvValue::Type(r) => r,
            _ => panic!(),
        })
    }

    pub(crate) fn register_value(
        &mut self,
        name_space: Vec<String>,
        name: String,
        type_: TypedType,
    ) {
        let entry = self
            .values
            .entry(name)
            .or_insert_with(|| EnvValue::from(HashSet::default()));
        if let EnvValue::Value(v) = entry {
            v.insert((name_space, type_));
        };
    }

    pub(crate) fn get_value(&self, name: &str) -> Option<&HashSet<(Vec<String>, TypedType)>> {
        match self.values.get(name) {
            None => None,
            Some(e) => match e {
                EnvValue::NameSpace(_) => None,
                EnvValue::Value(v) => Some(v),
                EnvValue::Type(_) => None,
            },
        }
    }
}
