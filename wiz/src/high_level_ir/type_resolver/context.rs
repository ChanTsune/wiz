use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_type::{Package, TypedType, TypedValueType};
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ResolverStruct {
    pub(crate) stored_properties: HashMap<String, TypedType>,
    pub(crate) computed_properties: HashMap<String, TypedType>,
    pub(crate) member_functions: HashMap<String, TypedType>,
    pub(crate) static_functions: HashMap<String, TypedType>,
    pub(crate) conformed_protocols: HashSet<String>,
    pub(crate) type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameSpace {
    name_space: Vec<String>,
    children: HashMap<String, NameSpace>,
    types: HashMap<String, ResolverStruct>,
    values: HashMap<String, TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameEnvironment {
    names: HashMap<String, (Vec<String>, EnvValue)>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum EnvValue {
    NameSpace(NameSpace),
    Value(TypedType),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ResolverSubscript {
    target: TypedType,
    indexes: Vec<TypedType>,
    return_type: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ResolverUnary {
    value: TypedType,
    return_type: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Debug, Clone)]
pub struct ResolverContext {
    name_space: NameSpace,
    binary_operators: HashMap<(BinaryOperator, TypedType, TypedType), TypedType>,
    subscripts: Vec<ResolverSubscript>,
    pub(crate) current_namespace: Vec<String>,
    current_type: Option<TypedType>,
    local_stack: StackedHashMap<String, EnvValue>,
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
    pub(crate) fn new(name: Vec<String>) -> Self {
        Self {
            name_space: name,
            children: Default::default(),
            types: Default::default(),
            values: Default::default(),
        }
    }

    pub(crate) fn get_child_mut(&mut self, mut ns: Vec<String>) -> Option<&mut NameSpace> {
        if ns.is_empty() {
            Some(self)
        } else {
            let n = ns.remove(0);
            let m = self.children.get_mut(&*n)?;
            m.get_child_mut(ns)
        }
    }

    pub(crate) fn set_child(&mut self, mut ns: Vec<String>) {
        if !ns.is_empty() {
            let n = &ns.remove(0);
            if !self.children.contains_key(n) {
                let mut name = self.name_space.clone();
                name.push(n.clone());
                self.children.insert(n.clone(), NameSpace::new(name));
            };
            self.children.get_mut(n).unwrap().set_child(ns);
        }
    }

    pub(crate) fn register_type(&mut self, name: String, s: ResolverStruct) {
        self.types.insert(name, s);
    }

    pub(crate) fn get_type_mut(&mut self, name: &String) -> Option<&mut ResolverStruct> {
        self.types.get_mut(name)
    }

    pub(crate) fn register_value(&mut self, name: String, type_: TypedType) {
        self.values.insert(name, type_);
    }
}

impl NameEnvironment {
    fn new() -> Self {
        Self {
            names: Default::default(),
        }
    }

    fn use_values_from(&mut self, name_space: &NameSpace) {
        self.names.extend(name_space.values.iter().map(|(k, v)| {
            (
                k.clone(),
                (name_space.name_space.clone(), EnvValue::from(v.clone())),
            )
        }));
        self.names.extend(name_space.children.iter().map(|(k, v)| {
            (
                k.clone(),
                (name_space.name_space.clone(), EnvValue::from(v.clone())),
            )
        }));
    }

    fn use_values_from_local(&mut self, local_stack: &StackedHashMap<String, EnvValue>) {
        self.names.extend(
            local_stack
                .clone()
                .into_map()
                .into_iter()
                .map(|(k, v)| (k, (vec![], v))),
        )
    }
}

impl From<TypedType> for EnvValue {
    fn from(typed_type: TypedType) -> Self {
        Self::Value(typed_type)
    }
}

impl From<NameSpace> for EnvValue {
    fn from(ns: NameSpace) -> Self {
        Self::NameSpace(ns)
    }
}

impl ResolverContext {
    pub(crate) fn new() -> Self {
        let mut ns = NameSpace::new(vec![]);

        let mut rs_for_pointer = ResolverStruct::new();
        let mut tp_map_for_pointer = HashMap::new();
        tp_map_for_pointer.insert(
            String::from("T"),
            ResolverTypeParam {
                type_constraints: vec![],
                type_params: None,
            },
        );
        rs_for_pointer.type_params = Some(tp_map_for_pointer);
        ns.types
            .insert(String::from(UNSAFE_POINTER), rs_for_pointer);

        for t in TypedType::builtin_types() {
            match &t {
                TypedType::Value(v) => {
                    ns.register_type(v.name.clone(), ResolverStruct::new());
                    ns.register_value(v.name.clone(), TypedType::Type(v.clone()));
                }
                _ => {}
            };
        }
        let mut bo = HashMap::new();
        for op in BinaryOperator::all() {
            for t in TypedType::integer_types() {
                bo.insert((op.clone(), t.clone(), t.clone()), t);
            }
            for t in TypedType::floating_point_types() {
                bo.insert((op.clone(), t.clone(), t.clone()), t);
            }
        }
        Self {
            name_space: ns,
            binary_operators: bo,
            subscripts: vec![],
            current_namespace: vec![],
            current_type: None,
            local_stack: StackedHashMap::new(),
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
        self.get_namespace_mut(self.current_namespace.clone())
    }

    fn get_namespace_mut(&mut self, ns: Vec<String>) -> Result<&mut NameSpace> {
        let msg = format!("NameSpace {:?} not exist", ns);
        self.name_space
            .get_child_mut(ns)
            .ok_or(ResolverError::from(msg))
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

    pub fn push_local_stack(&mut self) {
        self.local_stack.push(HashMap::new());
    }

    pub fn pop_local_stack(&mut self) {
        self.local_stack.pop();
    }

    pub fn clear_local_stack(&mut self) {
        self.local_stack = StackedHashMap::new()
    }

    pub(crate) fn register_to_env(&mut self, name: String, value: EnvValue) {
        if self.local_stack.stack_is_empty() {
            let ns = self.get_current_namespace_mut().unwrap();
            match value {
                EnvValue::NameSpace(n) => {
                    todo!()
                }
                EnvValue::Value(v) => ns.register_value(name, v),
            }
        } else {
            self.local_stack.insert(name, value);
        }
    }

    pub fn get_current_name_environment(&mut self) -> NameEnvironment {
        let mut env = NameEnvironment::new();
        env.use_values_from(self.get_namespace_mut(vec![]).unwrap());
        env.use_values_from(self.get_current_namespace_mut().unwrap());
        env.use_values_from_local(&self.local_stack);
        env
    }

    pub fn resolve_member_type(&mut self, t: TypedType, name: String) -> Result<TypedType> {
        match &t {
            TypedType::Value(v) => {
                let ns = self.get_namespace_mut(v.package.clone().unwrap().names)?;
                let rs = ns
                    .types
                    .get(&v.name)
                    .ok_or(ResolverError::from(format!("Can not resolve type {:?}", t)))?;
                rs.stored_properties
                    .get(&name)
                    .map(|it| it.clone())
                    .ok_or(ResolverError::from(format!("{:?} not has {:?}", t, name)))
            }
            TypedType::Type(v) => {
                let ns = self.get_namespace_mut(v.package.clone().unwrap().names)?;
                let rs = ns
                    .types
                    .get(&v.name)
                    .ok_or(ResolverError::from(format!("Can not resolve type {:?}", t)))?;
                rs.static_functions
                    .get(&name)
                    .map(|it| it.clone())
                    .ok_or(ResolverError::from(format!("{:?} not has {:?}", t, name)))
            }
            _ => todo!("dose not impl"),
        }
    }

    pub fn resolve_name_type(
        &mut self,
        name_space: Vec<String>,
        name: String,
    ) -> Result<(TypedType, Option<Package>)> {
        if !name_space.is_empty() {
            let ns = self.get_namespace_mut(name_space.clone())?;
            return Result::Ok((
                ns.values
                    .get(&name)
                    .ok_or(ResolverError::from(format!(
                        "Cannot resolve name {:?}",
                        name
                    )))?
                    .clone(),
                Some(Package::new(name_space)),
            ));
        }
        let env = self.get_current_name_environment();
        let env_value = env.names.get(&name).ok_or(ResolverError::from(format!(
            "Cannot resolve name {:?}",
            name
        )))?;
        match env_value {
            (ns, EnvValue::NameSpace(child)) => {
                todo!("{:?}", child)
            }
            (ns, EnvValue::Value(t)) => Result::Ok((
                t.clone(),
                if t.is_function_type() {
                    Some(Package::new(ns.clone()))
                } else {
                    None
                },
            )),
        }
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
            let ns = self.get_namespace_mut(cns.clone())?;
            match &typ {
                TypedType::Value(v) | TypedType::Reference(v) => {
                    if let Some(_) = ns.types.get(&v.name) {
                        return Result::Ok(TypedType::Value(TypedValueType {
                            package: Some(Package { names: cns.clone() }),
                            name: v.name.clone(),
                            type_args: v.type_args.clone(),
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

#[cfg(test)]
mod tests {
    use crate::high_level_ir::type_resolver::context::{EnvValue, NameSpace, ResolverContext};
    use crate::high_level_ir::typed_type::{Package, TypedType, TypedValueType};

    #[test]
    fn test_name_space() {
        let mut name_space = NameSpace::new(vec![]);
        name_space
            .values
            .insert(String::from("Int64"), TypedType::int64());
        name_space.set_child(vec![String::from("builtin")]);
        assert_eq!(
            name_space.get_child_mut(vec![String::from("builtin")]),
            Some(&mut NameSpace::new(vec![String::from("builtin")]))
        );
    }

    #[test]
    fn test_name_space_child_name_space() {
        let mut name_space = NameSpace::new(vec![]);
        name_space.set_child(vec![String::from("child")]);
        let ns = name_space
            .get_child_mut(vec![String::from("child")])
            .unwrap();
        assert_eq!(ns.name_space, vec![String::from("child")]);
    }

    #[test]
    fn test_name_space_grandchild_name_space() {
        let mut name_space = NameSpace::new(vec![]);
        name_space.set_child(vec![String::from("child"), String::from("grandchild")]);
        let ns = name_space
            .get_child_mut(vec![String::from("child"), String::from("grandchild")])
            .unwrap();
        assert_eq!(
            ns.name_space,
            vec![String::from("child"), String::from("grandchild")]
        );
    }

    #[test]
    fn test_name_space_grate_grandchild_name_space() {
        let mut name_space = NameSpace::new(vec![]);
        name_space.set_child(vec![
            String::from("child"),
            String::from("grandchild"),
            String::from("grate-grandchild"),
        ]);
        let ns = name_space
            .get_child_mut(vec![
                String::from("child"),
                String::from("grandchild"),
                String::from("grate-grandchild"),
            ])
            .unwrap();
        assert_eq!(
            ns.name_space,
            vec![
                String::from("child"),
                String::from("grandchild"),
                String::from("grate-grandchild")
            ]
        );
    }

    #[test]
    fn test_context_name_environment() {
        let mut context = ResolverContext::new();

        let env = context.get_current_name_environment();

        assert_eq!(
            env.names.get("Int32"),
            Some(&(
                vec![],
                EnvValue::Value(TypedType::Type(TypedValueType {
                    package: Some(Package::global()),
                    name: "Int32".to_string(),
                    type_args: None
                }))
            ))
        );
    }
}
