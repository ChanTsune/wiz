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
                EnvValue::NameSpace(m) => m.get_child(ns),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => None,
            }
        }
    }

    pub(crate) fn get_child_mut<T: ToString>(&mut self, mut ns: Vec<T>) -> Option<&mut NameSpace> {
        if ns.is_empty() {
            Some(self)
        } else {
            let n = ns.remove(0).to_string();
            let m = self.values.get_mut(&*n)?;
            match m {
                EnvValue::NameSpace(m) => m.get_child_mut(ns),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => panic!(),
            }
        }
    }

    pub(crate) fn set_child<T: ToString>(&mut self, mut ns: Vec<T>) {
        if !ns.is_empty() {
            let n = ns.remove(0).to_string();
            let self_name_space_ref =  &self.name_space;
            let entry = self.values.entry(n).or_insert_with_key(|key|{
                let mut name = self_name_space_ref.clone();
                name.push(key.clone());
                EnvValue::from(NameSpace::new(name))
            });
            match entry {
                EnvValue::NameSpace(n) => n.set_child(ns),
                EnvValue::Value(_) => panic!(),
                EnvValue::Type(_) => panic!(),
            };
        }
    }

    pub(crate) fn get<T: ToString>(&self, mut ns: Vec<T>) -> Option<&EnvValue> {
        let name = ns.remove(0).to_string();
        let e = self.values.get(&name)?;
        e.get(ns)
    }

    pub(crate) fn get_mut<T: ToString>(&mut self, mut ns: Vec<T>) -> Option<&mut EnvValue> {
        let name = ns.remove(0).to_string();
        let e = self.values.get_mut(&name)?;
        e.get_mut(ns)
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

    pub(crate) fn register_value(&mut self, name: String, type_: TypedType) {
        if let Some(e) = self.values.remove(&name) {
            match e {
                EnvValue::NameSpace(_) | EnvValue::Type(_) => {
                    self.values.insert(name, e);
                }
                EnvValue::Value(mut v) => {
                    v.insert(type_);
                    self.values.insert(name, EnvValue::from(v));
                }
            };
        } else {
            self.values
                .insert(name, EnvValue::from(HashSet::from([type_])));
        }
    }

    pub(crate) fn register_values(&mut self, name: String, type_: HashSet<TypedType>) {
        if let Some(e) = self.values.remove(&name) {
            match e {
                EnvValue::NameSpace(_) | EnvValue::Type(_) => {
                    self.values.insert(name, e);
                }
                EnvValue::Value(mut v) => {
                    v.extend(type_);
                    self.values.insert(name, EnvValue::from(v));
                }
            };
        } else {
            self.values.insert(name, EnvValue::from(type_));
        }
    }

    pub(crate) fn get_value(&self, name: &str) -> Option<&HashSet<TypedType>> {
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
