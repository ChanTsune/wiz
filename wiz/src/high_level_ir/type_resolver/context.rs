use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_type::{Package, TypedType, TypedValueType};
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

impl From<&str> for BinaryOperator {
    fn from(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            _ => panic!("Undefined op kind {:?}", op),
        }
    }
}

impl BinaryOperator {
    fn all() -> Vec<BinaryOperator> {
        vec![Self::Add, Self::Sub, Self::Mul, Self::Div, Self::Mod]
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverContext {
    name_space: NameSpace,
    binary_operators: HashMap<(BinaryOperator, TypedType, TypedType), TypedType>,
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
                _ => {}
            };
        }
        let mut bo = HashMap::new();
        for op in BinaryOperator::all() {
            for t in TypedType::integer_types() {
                bo.insert((op.clone(), t.clone(), t.clone()), t);
            }
        }
        Self {
            name_space: ns,
            binary_operators: bo,
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

    pub fn get_current_namespace_mut(&mut self) -> Result<&mut NameSpace> {
        self.name_space
            .get_child_mut(self.current_namespace.clone()).ok_or(ResolverError::from(format!("NameSpace {:?} not exist", self.current_namespace)))
    }

    pub fn get_namespace_mut(&mut self, ns: Vec<String>) -> Result<&mut NameSpace> {
        self.name_space.get_child_mut(ns.clone()).ok_or(ResolverError::from(format!("NameSpace {:?} not exist", ns)))
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

    pub fn resolve_member_type(&mut self, t: TypedType, name: String) -> Result<TypedType> {
        match &t {
            TypedType::Value(v) => {
                let ns = self
                    .get_namespace_mut(v.package.names.clone())?;
                println!("ns => {:?}", ns);
                let rs = ns
                    .types
                    .get(&v.name)
                    .ok_or(ResolverError::from(format!("Can not resolve type {:?}", t)))?;
                rs.stored_properties
                    .get(&name)
                    .map(|it| it.clone())
                    .ok_or(ResolverError::from(format!("{:?} not has {:?}", t, name)))
            }
            _ => todo!("dose not impl"),
        }
    }

    pub fn resolve_name_type(&mut self, name: String) -> Result<TypedType> {
        let mut cns = self.current_namespace.clone();
        loop {
            let ns = self
                .get_namespace_mut(cns.clone())?;
            if let Some(t) = ns.values.get(&name) {
                return Result::Ok(t.clone());
            }
            if cns.is_empty() {
                break;
            }
            cns.pop();
        }
        Result::Err(ResolverError::from(format!(
            "Cannot resolve name {:?}",
            name
        )))
    }

    pub fn resolve_binop_type(
        &self,
        left: TypedType,
        kind: &str,
        right: TypedType,
    ) -> Result<TypedType> {
        match kind {
            "<" | "<=" | ">" | ">=" | "==" | "!=" => Result::Ok(TypedType::bool()),
            _ => {
                let op_kind = BinaryOperator::from(kind);
                let key = (op_kind, left, right);
                self.binary_operators
                    .get(&key)
                    .map(|t| t.clone())
                    .ok_or(ResolverError::from(format!("{:?} is not defined.", key)))
            }
        }
    }

    pub fn full_type_name(&mut self, typ: TypedType) -> Result<TypedType> {
        // TODO: change impl
        if typ.is_primitive() {
            return Result::Ok(typ);
        };
        let mut cns = self.current_namespace.clone();
        loop {
            let ns = self
                .get_namespace_mut(cns.clone())?;
            match &typ {
                TypedType::Value(v) => {
                    if let Some(_) = ns.types.get(&v.name) {
                        return Result::Ok(TypedType::Value(TypedValueType {
                            package: Package { names: cns.clone() },
                            name: v.name.clone(),
                            type_args: None,
                        }));
                    };
                }
                _ => {
                    todo!("Dose not impl")
                }
            }
            if cns.is_empty() {
                break;
            }
            cns.pop();
        }
        Result::Err(ResolverError::from(format!(
            "Type {:?} dose not exist",
            typ
        )))
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
