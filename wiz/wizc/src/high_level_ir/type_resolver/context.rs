mod env_value;
mod resolver_struct;

use crate::high_level_ir::type_resolver::arena::ResolverArena;
pub(crate) use crate::high_level_ir::type_resolver::context::env_value::EnvValue;
pub(crate) use crate::high_level_ir::type_resolver::context::resolver_struct::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::name_environment::NameEnvironment;
use crate::high_level_ir::type_resolver::namespace::NameSpace;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedValueType,
};
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ResolverContext {
    global_used_name_space: Vec<Vec<String>>,
    used_name_space: Vec<Vec<String>>,
    arena: ResolverArena,
    pub(crate) current_namespace: Vec<String>,
    current_type: Option<TypedType>,
    local_stack: StackedHashMap<String, EnvValue>,
}

impl ResolverContext {
    pub(crate) fn new() -> Self {
        Self {
            global_used_name_space: Default::default(),
            used_name_space: Default::default(),
            arena: ResolverArena::default(),
            current_namespace: Default::default(),
            current_type: None,
            local_stack: StackedHashMap::new(),
        }
    }

    pub fn push_name_space(&mut self, name: String) {
        self.current_namespace.push(name);
        self.arena
            .name_space
            .set_child(self.current_namespace.clone());
    }

    pub fn pop_name_space(&mut self) {
        self.current_namespace.pop();
    }

    pub fn get_current_namespace_mut(&mut self) -> Result<&mut NameSpace> {
        self.get_namespace_mut(self.current_namespace.clone())
    }

    pub fn get_namespace_mut(&mut self, ns: Vec<String>) -> Result<&mut NameSpace> {
        let msg = format!("NameSpace {:?} not exist", ns);
        self.arena
            .name_space
            .get_child_mut(ns)
            .ok_or_else(|| ResolverError::from(msg))
    }

    pub fn get_current_namespace(&self) -> Result<&NameSpace> {
        self.get_namespace(self.current_namespace.clone())
    }

    pub fn get_namespace(&self, ns: Vec<String>) -> Result<&NameSpace> {
        let msg = format!("NameSpace {:?} not exist", ns);
        self.arena
            .name_space
            .get_child(ns)
            .ok_or_else(|| ResolverError::from(msg))
    }

    pub fn resolve_current_type(&self) -> Result<TypedType> {
        self.current_type
            .clone()
            .ok_or_else(|| ResolverError::from("can not resolve Self"))
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

    pub(crate) fn register_to_env<T>(&mut self, name: String, value: T)
    where
        EnvValue: From<T>,
    {
        let value = EnvValue::from(value);
        if self.local_stack.stack_is_empty() {
            let ns = self.get_current_namespace_mut().unwrap();
            match value {
                EnvValue::NameSpace(n) => {
                    todo!()
                }
                EnvValue::Value(v) => ns.register_values(name, v),
                EnvValue::Type(_) => {
                    ns.values.insert(name, value);
                }
            };
        } else {
            self.local_stack.insert(name, value);
        }
    }

    pub(crate) fn get_current_name_environment(&self) -> NameEnvironment {
        let mut env = NameEnvironment::new();
        env.use_values_from(self.get_namespace(vec![]).unwrap());
        env.use_values_from(self.get_current_namespace().unwrap());
        let used_ns = self
            .global_used_name_space
            .iter()
            .cloned()
            .map(|ns| (true, ns))
            .chain(self.used_name_space.iter().cloned().map(|ns| (false, ns)));
        for (is_global, mut u) in used_ns {
            if u.last().is_some() && u.last().unwrap() == "*" {
                let _ = u.pop();
                if let Ok(n) = self.get_namespace(u.clone()) {
                    env.use_values_from(n);
                } else if !is_global {
                    panic!("Can not resolve name space {:?}", u);
                }
            } else {
                if let Ok(n) = self.get_namespace(u.clone()) {
                    let name = u.pop().unwrap();
                    env.names.insert(name, (u, EnvValue::from(n.clone())));
                } else {
                    let name = u.pop().unwrap();
                    let s = self.get_namespace(u.clone()).unwrap();
                    if let Some(t) = s.values.get(&name) {
                        env.names.insert(name, (u, t.clone()));
                    } else if !is_global {
                        panic!("Can not find name {:?}", name)
                    };
                };
            }
        }
        env.use_values_from_local(&self.local_stack);
        env
    }

    pub(crate) fn global_use_name_space(&mut self, ns: Vec<String>) {
        self.global_used_name_space.push(ns);
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

    pub fn resolve_member_type(&mut self, t: TypedType, name: &str) -> Result<TypedType> {
        match &t {
            TypedType::Value(v) => match v {
                TypedValueType::Value(v) => {
                    let ne = self.get_current_name_environment();
                    let rs = ne
                        .get_type(v.package.clone().into_resolved().names, &v.name)
                        .ok_or_else(|| {
                            ResolverError::from(format!("Can not resolve type {:?}", t))
                        })?;
                    rs.get_instance_member_type(&name).cloned().ok_or_else(|| {
                        ResolverError::from(format!("{:?} not has member named `{}`", t, name))
                    })
                }
                TypedValueType::Array(_, _) => {
                    todo!()
                }
                TypedValueType::Tuple(_) => {
                    todo!()
                }
                TypedValueType::Pointer(_) => {
                    todo!()
                }
                TypedValueType::Reference(_) => {
                    todo!()
                }
            },
            TypedType::Type(v) => match &**v {
                TypedType::Self_ => {
                    todo!()
                }
                TypedType::Value(v) => match v {
                    TypedValueType::Value(v) => {
                        let ns = self.get_namespace_mut(v.package.clone().into_resolved().names)?;
                        let rs = ns.get_type(&v.name).ok_or_else(|| {
                            ResolverError::from(format!("Can not resolve type {:?}", t))
                        })?;
                        rs.static_functions.get(name).cloned().ok_or_else(|| {
                            ResolverError::from(format!(
                                "{:?} not has static member named `{}`",
                                t, name
                            ))
                        })
                    }
                    TypedValueType::Array(_, _) => {
                        todo!()
                    }
                    TypedValueType::Tuple(_) => {
                        todo!()
                    }
                    TypedValueType::Pointer(_) => {
                        todo!()
                    }
                    TypedValueType::Reference(_) => {
                        todo!()
                    }
                },
                TypedType::Function(_) => {
                    todo!()
                }
                TypedType::Type(_) => {
                    todo!()
                }
            },
            _ => todo!("dose not impl"),
        }
    }

    pub fn resolve_name_type(
        &mut self,
        mut name_space: Vec<String>,
        name: String,
        type_annotation: Option<TypedType>,
    ) -> Result<(TypedType, TypedPackage)> {
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
                let ns = child.get_child(name_space.clone()).ok_or_else(|| {
                    ResolverError::from(format!("Cannot resolve namespace {:?}", name_space))
                })?;
                let t_set = ns
                    .get_value(&n)
                    .ok_or_else(|| ResolverError::from(format!("Cannot resolve name {:?}", n)))?;
                Self::resolve_overload(t_set, type_annotation)
                    .map(|t| {
                        let is_function = t.is_function_type();
                        (
                            t,
                            if is_function {
                                TypedPackage::Resolved(Package::from(ns.name_space.clone()))
                            } else {
                                TypedPackage::Resolved(Package::global())
                            },
                        )
                    })
                    .ok_or_else(|| {
                        ResolverError::from(format!(
                            "Dose not match any overloaded function `{}`",
                            n
                        ))
                    })
            }
            (ns, EnvValue::Value(t_set)) => Self::resolve_overload(t_set, type_annotation)
                .map(|t| {
                    let is_function = t.is_function_type();
                    (
                        t,
                        if is_function {
                            TypedPackage::Resolved(Package::from(ns.clone()))
                        } else {
                            TypedPackage::Resolved(Package::global())
                        },
                    )
                })
                .ok_or_else(|| {
                    ResolverError::from(format!(
                        "Dose not match any overloaded function `{}`",
                        name
                    ))
                }),
            (_, EnvValue::Type(rs)) => Ok((
                TypedType::Type(Box::new(rs.self_.clone())),
                TypedPackage::Resolved(Package::global()),
            )),
        }
    }

    fn resolve_overload(
        type_set: &HashSet<TypedType>,
        type_annotation: Option<TypedType>,
    ) -> Option<TypedType> {
        for t in type_set {
            if type_set.len() == 1 {
                return Some(t.clone());
            } else if let Some(TypedType::Function(annotation)) = &type_annotation {
                if let TypedType::Function(typ) = t {
                    if annotation.arguments == typ.arguments {
                        return Some(t.clone());
                    }
                }
            }
        }
        None
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
            | TypedBinaryOperator::NotEqual => Ok(TypedType::bool()),
            TypedBinaryOperator::InfixFunctionCall(op) => {
                todo!("InfixFunctionCall => {}", op)
            }
            kind => {
                let is_both_integer = left.is_integer() && right.is_integer();
                let is_both_float = left.is_floating_point() && right.is_floating_point();
                let is_both_same = left == right;
                let is_pointer_op = left.is_pointer_type() && right.is_integer();
                if (is_both_same && (is_both_integer || is_both_float)) || is_pointer_op {
                    Ok(left)
                } else {
                    let key = (kind, left, right);
                    self.arena
                        .binary_operators
                        .get(&key)
                        .cloned()
                        .ok_or_else(|| ResolverError::from(format!("{:?} is not defined.", key)))
                }
            }
        }
    }

    fn full_value_type_name(&self, type_: TypedValueType) -> Result<TypedValueType> {
        Ok(match type_ {
            TypedValueType::Value(t) => TypedValueType::Value(self.full_named_value_type_name(t)?),
            TypedValueType::Array(a, n) => {
                TypedValueType::Array(Box::new(self.full_type_name(*a)?), n)
            }
            TypedValueType::Tuple(_) => {
                todo!()
            }
            TypedValueType::Pointer(t) => {
                TypedValueType::Pointer(Box::new(self.full_type_name(*t)?))
            }
            TypedValueType::Reference(t) => {
                TypedValueType::Reference(Box::new(self.full_type_name(*t)?))
            }
        })
    }

    fn full_named_value_type_name(
        &self,
        type_: TypedNamedValueType,
    ) -> Result<TypedNamedValueType> {
        let env = self.get_current_name_environment();
        Ok(match type_.package {
            TypedPackage::Raw(p) => {
                if p.names.is_empty() {
                    let (ns, t) =
                        env.names
                            .get(&type_.name)
                            .ok_or(ResolverError::from(format!(
                                "Can not resolve name `{}`",
                                &type_.name
                            )))?;
                    TypedNamedValueType {
                        package: TypedPackage::Resolved(Package::from(ns.clone())),
                        name: type_.name.clone(),
                        type_args: match type_.type_args.clone() {
                            None => None,
                            Some(v) => Some(
                                v.into_iter()
                                    .map(|i| self.full_type_name(i))
                                    .collect::<Result<Vec<_>>>()?,
                            ),
                        },
                    }
                } else {
                    let mut name_space = p.names;
                    let name = name_space.remove(0);
                    let env_value = env.names.get(&name).ok_or_else(|| {
                        ResolverError::from(format!("Cannot resolve name => {:?}", name))
                    })?;
                    match env_value {
                        (_, EnvValue::NameSpace(child)) => {
                            let ns = child.get_child(name_space.clone()).ok_or_else(|| {
                                ResolverError::from(format!(
                                    "Cannot resolve namespace {:?}",
                                    name_space
                                ))
                            })?;
                            let _ = ns.get_type(&type_.name).ok_or(ResolverError::from(
                                format!("Cannot resolve name {:?}", &type_.name),
                            ))?;
                            TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(
                                    ns.name_space.clone(),
                                )),
                                name: type_.name.clone(),
                                type_args: match type_.type_args.clone() {
                                    None => None,
                                    Some(v) => Some(
                                        v.into_iter()
                                            .map(|i| self.full_type_name(i))
                                            .collect::<Result<Vec<TypedType>>>()?,
                                    ),
                                },
                            }
                        }
                        (ns, EnvValue::Value(t)) => panic!(),
                        (_, _) => todo!(),
                    }
                }
            }
            TypedPackage::Resolved(_) => type_,
        })
    }

    pub fn full_type_name(&self, typ: TypedType) -> Result<TypedType> {
        if typ.is_self() {
            self.resolve_current_type()
        } else {
            Ok(match typ {
                TypedType::Value(v) => TypedType::Value(self.full_value_type_name(v)?),
                TypedType::Type(v) => TypedType::Type(Box::new(self.full_type_name(*v)?)),
                TypedType::Self_ => self.resolve_current_type()?,
                TypedType::Function(f) => TypedType::Function(Box::new(TypedFunctionType {
                    arguments: f
                        .arguments
                        .into_iter()
                        .map(|a| {
                            Ok(TypedArgType {
                                label: a.label,
                                typ: self.full_type_name(a.typ)?,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                    return_type: self.full_type_name(f.return_type)?,
                })),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::high_level_ir::type_resolver::context::{
        EnvValue, NameSpace, ResolverContext, ResolverStruct,
    };
    use crate::high_level_ir::typed_type::TypedType;

    #[test]
    fn test_name_space() {
        let mut name_space = NameSpace::empty();
        name_space
            .values
            .insert(String::from("Int64"), EnvValue::from(TypedType::int64()));
        name_space.set_child(vec!["builtin"]);
        assert_eq!(
            name_space.get_child_mut(vec!["builtin"]),
            Some(&mut NameSpace::new(vec!["builtin"]))
        );
    }

    #[test]
    fn test_name_space_child_name_space() {
        let mut name_space = NameSpace::empty();
        name_space.set_child(vec!["child"]);
        let ns = name_space.get_child_mut(vec!["child"]).unwrap();
        assert_eq!(ns.name_space, vec!["child"]);
    }

    #[test]
    fn test_name_space_grandchild_name_space() {
        let mut name_space = NameSpace::empty();
        name_space.set_child(vec!["child", "grandchild"]);
        let ns = name_space
            .get_child_mut(vec!["child", "grandchild"])
            .unwrap();
        assert_eq!(ns.name_space, vec!["child", "grandchild"]);
    }

    #[test]
    fn test_name_space_grate_grandchild_name_space() {
        let mut name_space = NameSpace::empty();
        name_space.set_child(vec!["child", "grandchild", "grate-grandchild"]);
        let ns = name_space
            .get_child_mut(vec!["child", "grandchild", "grate-grandchild"])
            .unwrap();
        assert_eq!(
            ns.name_space,
            vec!["child", "grandchild", "grate-grandchild"]
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
                EnvValue::Type(ResolverStruct::new(TypedType::int32()))
            ))
        );
    }
}
