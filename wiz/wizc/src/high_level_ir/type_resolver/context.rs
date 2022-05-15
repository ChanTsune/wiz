mod env_value;
mod resolver_struct;

use crate::high_level_ir::type_resolver::arena::ResolverArena;
pub(crate) use crate::high_level_ir::type_resolver::context::env_value::EnvValue;
pub(crate) use crate::high_level_ir::type_resolver::context::resolver_struct::{
    ResolverStruct, StructKind,
};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::name_environment::NameEnvironment;
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
    pub(crate) arena: ResolverArena,
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

    pub(crate) fn arena(&self) -> &ResolverArena {
        &self.arena
    }

    pub fn push_name_space(&mut self, name: String) {
        self.arena
            .register_namespace(&self.current_namespace, &name, Default::default());
        self.current_namespace.push(name);
    }

    pub fn pop_name_space(&mut self) {
        self.current_namespace.pop();
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
            match value {
                EnvValue::NameSpace(_) => todo!(),
                EnvValue::Value(v) => {
                    for t in v {
                        self.arena.register_value(
                            &self.current_namespace,
                            &name,
                            t.1,
                            Default::default(),
                        );
                    }
                }
                EnvValue::Type(_) => todo!(),
            };
        } else {
            self.local_stack.insert(name, value);
        }
    }

    pub(crate) fn get_current_name_environment(&self) -> NameEnvironment {
        let mut env = NameEnvironment::new(&self.arena);
        let root_namespace_name: [&str; 0] = [];
        env.use_asterisk(&root_namespace_name);

        env.use_asterisk(&self.current_namespace);
        let used_ns = self
            .global_used_name_space
            .iter()
            .map(|ns| (true, ns))
            .chain(self.used_name_space.iter().map(|ns| (false, ns)));
        for (is_global, u) in used_ns {
            env.use_(u);
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
                    rs.get_instance_member_type(name).cloned().ok_or_else(|| {
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
            TypedType::Type(v) => Err(ResolverError::from(format!(
                "{:?} has no member {}",
                v, name
            ))),
            _ => todo!("dose not impl"),
        }
    }

    pub fn resolve_name_type(
        &mut self,
        name_space: Vec<String>,
        name: &str,
        type_annotation: Option<TypedType>,
    ) -> Result<(TypedType, TypedPackage)> {
        if name_space.is_empty() && name == "Self" {
            let self_type = self.resolve_current_type()?;
            let package = self_type.package();
            return Ok((TypedType::Type(Box::new(self_type)), package));
        }
        let env = self.get_current_name_environment();
        let env_value = env.get_env_item(&name_space, name).ok_or_else(|| {
            ResolverError::from(format!("Cannot resolve name =>{:?} {:?}", name_space, name))
        })?;
        match env_value {
            EnvValue::NameSpace(_) => unreachable!(),
            EnvValue::Value(t_set) => Self::resolve_overload(&t_set, type_annotation)
                .map(|(ns, t)| (t, TypedPackage::Resolved(Package::from(&ns))))
                .ok_or_else(|| {
                    ResolverError::from(format!(
                        "Dose not match any overloaded function `{}`",
                        name
                    ))
                }),
            EnvValue::Type(rs) => Ok((
                TypedType::Type(Box::new(rs.self_type())),
                rs.self_type().package(),
            )),
        }
    }

    fn resolve_overload(
        type_set: &HashSet<(Vec<String>, TypedType)>,
        type_annotation: Option<TypedType>,
    ) -> Option<(Vec<String>, TypedType)> {
        for t in type_set {
            if type_set.len() == 1 {
                return Some(t.clone());
            } else if let Some(TypedType::Function(annotation)) = &type_annotation {
                if let (_, TypedType::Function(typ)) = t {
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
                        .resolve_binary_operator(&key)
                        .cloned()
                        .ok_or_else(|| ResolverError::from(format!("{:?} is not defined.", key)))
                }
            }
        }
    }

    fn full_value_type_name(&self, type_: TypedValueType) -> Result<TypedValueType> {
        Ok(match type_ {
            TypedValueType::Value(t) => TypedValueType::Value(self.full_named_value_type_name(&t)?),
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
        type_: &TypedNamedValueType,
    ) -> Result<TypedNamedValueType> {
        let env = self.get_current_name_environment();
        Ok(match type_.package {
            TypedPackage::Raw(ref p) => {
                let env_value = env.get_env_item(&p.names, &type_.name).ok_or_else(|| {
                    ResolverError::from(format!(
                        "Cannot resolve name => {:?}{}",
                        &p.names, &type_.name
                    ))
                })?;
                match env_value {
                    EnvValue::Type(rs) => TypedNamedValueType {
                        package: TypedPackage::Resolved(Package::from(&rs.namespace)),
                        name: type_.name.clone(),
                        type_args: match type_.type_args.clone() {
                            None => None,
                            Some(v) => Some(
                                v.into_iter()
                                    .map(|i| self.full_type_name(i))
                                    .collect::<Result<Vec<_>>>()?,
                            ),
                        },
                    },
                    _ => panic!(),
                }
            }
            TypedPackage::Resolved(_) => type_.clone(),
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
    use super::{ResolverContext, ResolverStruct, StructKind};
    use crate::constants::INT32;
    use crate::high_level_ir::typed_type::TypedType;

    #[test]
    fn test_context_name_environment() {
        let mut context = ResolverContext::new();

        let env = context.get_current_name_environment();

        assert_eq!(
            env.get_type(vec![], INT32),
            Some(&ResolverStruct::new(TypedType::int32(), StructKind::Struct)),
        );
    }
}
