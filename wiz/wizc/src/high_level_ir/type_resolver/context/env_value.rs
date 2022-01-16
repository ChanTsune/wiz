use crate::high_level_ir::type_resolver::context::{NameSpace, ResolverStruct};
use crate::high_level_ir::typed_type::TypedType;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum EnvValue {
    NameSpace(NameSpace),
    Value(HashSet<TypedType>),
    Type(ResolverStruct),
}

impl EnvValue {
    pub(crate) fn get<T>(&self, mut ns: Vec<T>) -> Option<&EnvValue>
    where
        T: ToString,
    {
        if ns.is_empty() {
            Some(self)
        } else {
            match self {
                EnvValue::NameSpace(n) => n.get(ns),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => todo!(),
            }
        }
    }

    pub(crate) fn get_mut<T>(&mut self, mut ns: Vec<T>) -> Option<&mut EnvValue>
    where
        T: ToString,
    {
        if ns.is_empty() {
            Some(self)
        } else {
            match self {
                EnvValue::NameSpace(n) => n.get_mut(ns),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => todo!(),
            }
        }
    }

    pub(crate) fn create_children<T: ToString>(&mut self, mut ns: Vec<T>) {
        if !ns.is_empty() {
            match self {
                EnvValue::NameSpace(n) => n.set_child(ns),
                EnvValue::Value(_) => panic!(),
                EnvValue::Type(_) => panic!(),
            }
        }
    }
}

impl From<TypedType> for EnvValue {
    fn from(typed_type: TypedType) -> Self {
        Self::Value(HashSet::from([typed_type]))
    }
}

impl From<NameSpace> for EnvValue {
    fn from(ns: NameSpace) -> Self {
        Self::NameSpace(ns)
    }
}

impl From<HashSet<TypedType>> for EnvValue {
    fn from(typed_type: HashSet<TypedType>) -> Self {
        Self::Value(typed_type)
    }
}

impl From<ResolverStruct> for EnvValue {
    fn from(s: ResolverStruct) -> Self {
        Self::Type(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::high_level_ir::type_resolver::context::{EnvValue, NameSpace};

    #[test]
    fn test_get() {
        let mut env_value = EnvValue::from(NameSpace::empty());
        env_value.create_children(vec!["child", "grand-child"]);
        assert_eq!(
            env_value.get(vec!["child", "grand-child"]),
            Some(&EnvValue::from(NameSpace::new(vec![
                "child",
                "grand-child"
            ])))
        );
    }
}
