use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_decl::{TypedArgDef, TypedValueArgDef};
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedValueType};
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::{HashMap, HashSet};
use std::option::Option::Some;

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

#[derive(Debug, Clone)]
pub struct ResolverContext {
    used_name_space: Vec<Vec<String>>,
    name_space: NameSpace,
    binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
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

    pub(crate) fn get_instance_member_type(&self, name: &String) -> Option<&TypedType> {
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

impl NameSpace {
    pub(crate) fn new(name: Vec<String>) -> Self {
        Self {
            name_space: name,
            children: Default::default(),
            types: Default::default(),
            values: Default::default(),
        }
    }

    pub(crate) fn get_child(&self, mut ns: Vec<String>) -> Option<&NameSpace> {
        if ns.is_empty() {
            Some(self)
        } else {
            let n = ns.remove(0);
            let m = self.children.get(&*n)?;
            m.get_child(ns)
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

    pub(crate) fn get_type(&self, name: &String) -> Option<&ResolverStruct> {
        self.types.get(name)
    }

    pub(crate) fn get_type_mut(&mut self, name: &String) -> Option<&mut ResolverStruct> {
        self.types.get_mut(name)
    }

    pub(crate) fn register_value(&mut self, name: String, type_: TypedType) {
        self.values.insert(name, type_);
    }

    pub(crate) fn get_value(&self, name: &String) -> Option<&TypedType> {
        self.values.get(name)
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
        ns.register_type(String::from(UNSAFE_POINTER), rs_for_pointer);

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
        for op in vec![
            TypedBinaryOperator::Add,
            TypedBinaryOperator::Sub,
            TypedBinaryOperator::Mul,
            TypedBinaryOperator::Div,
            TypedBinaryOperator::Mod,
        ] {
            for t in TypedType::integer_types() {
                bo.insert((op.clone(), t.clone(), t.clone()), t);
            }
            for t in TypedType::floating_point_types() {
                bo.insert((op.clone(), t.clone(), t.clone()), t);
            }
        }
        Self {
            used_name_space: Default::default(),
            name_space: ns,
            binary_operators: bo,
            subscripts: Default::default(),
            current_namespace: Default::default(),
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

    pub fn get_current_namespace(&self) -> Result<&NameSpace> {
        self.get_namespace(self.current_namespace.clone())
    }

    fn get_namespace(&self, ns: Vec<String>) -> Result<&NameSpace> {
        let msg = format!("NameSpace {:?} not exist", ns);
        self.name_space
            .get_child(ns)
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

    pub fn get_current_name_environment(&self) -> NameEnvironment {
        let mut env = NameEnvironment::new();
        env.use_values_from(self.get_namespace(vec![]).unwrap());
        env.use_values_from(self.get_current_namespace().unwrap());
        for mut u in self.used_name_space.iter().cloned() {
            if let Result::Ok(n) = self.get_namespace(u.clone()) {
                let name = u.pop().unwrap();
                env.names.insert(name, (u, EnvValue::NameSpace(n.clone())));
            } else {
                let name = u.pop().unwrap();
                let s = self.get_namespace(u.clone()).unwrap();
                let t = s.get_value(&name);
                env.names
                    .insert(name, (u, EnvValue::Value(t.cloned().unwrap())));
            };
        }
        env.use_values_from_local(&self.local_stack);
        env
    }

    pub(crate) fn use_name_space(&mut self, n: Vec<String>) {
        self.used_name_space.push(n);
    }

    pub(crate) fn unuse_name_space(&mut self, n: Vec<String>) {
        let i = self.used_name_space.iter().rposition(|i| i.eq(&n));
        if let Some(i) = i {
            self.used_name_space.remove(i);
        };
    }

    pub fn resolve_member_type(&mut self, t: TypedType, name: String) -> Result<TypedType> {
        match &t {
            TypedType::Value(v) => {
                let ns = self.get_namespace_mut(v.package.clone().unwrap().names)?;
                let rs = ns
                    .get_type(&v.name)
                    .ok_or(ResolverError::from(format!("Can not resolve type {:?}", t)))?;
                rs.get_instance_member_type(&name)
                    .map(|it| it.clone())
                    .ok_or(ResolverError::from(format!("{:?} not has {:?}", t, name)))
            }
            TypedType::Type(v) => {
                let ns = self.get_namespace_mut(v.package.clone().unwrap().names)?;
                let rs = ns
                    .get_type(&v.name)
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
        mut name_space: Vec<String>,
        name: String,
    ) -> Result<(TypedType, Option<Package>)> {
        let (name, name_space, n) = if name_space.is_empty() {
            (name, name_space, None)
        } else {
            (name_space.remove(0), name_space, Some(name))
        };
        let env = self.get_current_name_environment();
        let env_value = env
            .names
            .get(&name)
            .ok_or_else(|| ResolverError::from(format!("Cannot resolve name => {:?}", name)))?;
        match env_value {
            (_, EnvValue::NameSpace(child)) => {
                let n = n.unwrap();
                let ns = child
                    .get_child(name_space.clone())
                    .ok_or(ResolverError::from(format!(
                        "Cannot resolve namespace {:?}",
                        name_space
                    )))?;
                let t = ns
                    .get_value(&n)
                    .ok_or(ResolverError::from(format!("Cannot resolve name {:?}", n)))?;
                Result::Ok((
                    t.clone(),
                    if t.is_function_type() {
                        Some(Package::new(ns.name_space.clone()))
                    } else {
                        None
                    },
                ))
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
        kind: TypedBinaryOperator,
        right: TypedType,
    ) -> Result<TypedType> {
        match kind {
            TypedBinaryOperator::Equal
            | TypedBinaryOperator::GrateThanEqual
            | TypedBinaryOperator::GrateThan
            | TypedBinaryOperator::LessThanEqual
            | TypedBinaryOperator::LessThan
            | TypedBinaryOperator::NotEqual => Result::Ok(TypedType::bool()),
            TypedBinaryOperator::InfixFunctionCall(_) => {
                todo!()
            }
            _ => {
                let key = (kind, left, right);
                self.binary_operators
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| ResolverError::from(format!("{:?} is not defined.", key)))
            }
        }
    }

    pub fn full_type_name(&self, typ: TypedType) -> Result<TypedType> {
        // TODO: change impl
        if typ.is_primitive() {
            return Result::Ok(typ);
        };
        let mut cns = self.current_namespace.clone();
        loop {
            let ns = self.get_namespace(cns.clone())?;
            match &typ {
                TypedType::Value(v) | TypedType::Reference(v) => {
                    if let Some(_) = ns.get_type(&v.name) {
                        return Result::Ok(TypedType::Value(TypedValueType {
                            package: Some(Package { names: cns.clone() }),
                            name: v.name.clone(),
                            type_args: match v.type_args.clone() {
                                None => None,
                                Some(v) => Some(
                                    v.into_iter()
                                        .map(|i| self.full_type_name(i))
                                        .collect::<Result<Vec<TypedType>>>()?,
                                ),
                            },
                        }));
                    };
                }
                TypedType::Function(f) => {
                    return Result::Ok(TypedType::Function(Box::new(TypedFunctionType {
                        arguments: f
                            .arguments
                            .clone()
                            .into_iter()
                            .map(|a| {
                                Result::Ok(match a {
                                    TypedArgDef::Value(v) => TypedArgDef::Value(TypedValueArgDef {
                                        label: v.label,
                                        name: v.name,
                                        type_: self.full_type_name(v.type_)?,
                                    }),
                                    TypedArgDef::Self_(_) => {
                                        TypedArgDef::Self_(self.current_type.clone())
                                    }
                                    TypedArgDef::RefSelf(_) => {
                                        TypedArgDef::RefSelf(self.current_type.clone())
                                    }
                                })
                            })
                            .collect::<Result<Vec<TypedArgDef>>>()?,
                        return_type: self.full_type_name(f.return_type.clone())?,
                    })))
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
