pub mod context;
pub mod error;
pub mod result;
#[cfg(test)]
mod tests;

use crate::high_level_ir::type_resolver::context::{ResolverContext, ResolverStruct};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedExtension, TypedFun, TypedFunBody, TypedInitializer,
    TypedMemberFunction, TypedProtocol, TypedStoredProperty, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedArray, TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember,
    TypedLiteral, TypedName, TypedPostfixUnaryOp, TypedPrefixUnaryOp, TypedPrefixUnaryOperator,
    TypedReturn, TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedValueType,
};

#[derive(Debug, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self::from(ResolverContext::new())
    }

    pub(crate) fn global_use<T>(&mut self, name_space: Vec<T>)
    where
        T: ToString,
    {
        self.context
            .use_name_space(name_space.into_iter().map(|n| n.to_string()).collect())
    }

    pub fn detect_type_from_source_set(&mut self, s: &TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.detect_type(f),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name.clone());
                items
                    .iter()
                    .map(|i| self.detect_type_from_source_set(i))
                    .collect::<Result<Vec<_>>>()?;
                self.context.pop_name_space();
                Result::Ok(())
            }
        }
    }

    pub fn detect_type(&mut self, f: &TypedFile) -> Result<()> {
        self.context.push_name_space(f.name.clone());
        let current_namespace = self.context.current_namespace.clone();
        let ns = self.context.get_current_namespace_mut()?;
        for d in f.body.iter() {
            match d {
                TypedDecl::Struct(s) => {
                    let self_type = TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                        package: TypedPackage::Resolved(Package::from(current_namespace.clone())),
                        name: s.name.clone(),
                        type_args: None,
                    }));
                    ns.register_type(s.name.clone(), ResolverStruct::new(self_type.clone()));
                    ns.register_value(s.name.clone(), TypedType::Type(Box::new(self_type)));
                }
                TypedDecl::Class => {}
                TypedDecl::Enum => {}
                TypedDecl::Protocol(p) => {
                    let self_type = TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                        package: TypedPackage::Resolved(Package::from(current_namespace.clone())),
                        name: p.name.clone(),
                        type_args: None,
                    }));
                    ns.register_type(p.name.clone(), ResolverStruct::new(self_type.clone()));
                    ns.register_value(p.name.clone(), TypedType::Type(Box::new(self_type)));
                }
                _ => {}
            }
        }
        self.context.pop_name_space();
        Result::Ok(())
    }

    pub fn preload_source_set(&mut self, s: TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.preload_file(f),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name);
                items
                    .into_iter()
                    .map(|i| self.preload_source_set(i))
                    .collect::<Result<Vec<()>>>()?;
                self.context.pop_name_space();
                Result::Ok(())
            }
        }
    }

    pub fn preload_file(&mut self, f: TypedFile) -> Result<()> {
        self.context.push_name_space(f.name.clone());
        for u in f.uses.iter() {
            self.context.use_name_space(u.package.names.clone());
        }
        for d in f.body {
            self.preload_decl(d)?;
        }
        for u in f.uses.iter() {
            self.context.use_name_space(u.package.names.clone());
        }
        self.context.pop_name_space();
        Result::Ok(())
    }

    fn preload_decl(&mut self, d: TypedDecl) -> Result<()> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.typed_var(v)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace.register_value(
                    v.name,
                    v.type_
                        .ok_or_else(|| ResolverError::from("Cannot resolve variable type"))?,
                );
            }
            TypedDecl::Fun(f) => {
                let fun = self.preload_fun(f)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace.register_value(fun.name.clone(), fun.type_().unwrap());
            }
            TypedDecl::Struct(s) => {
                let _ = self.preload_struct(s)?;
            }
            TypedDecl::Class => todo!(),
            TypedDecl::Enum => todo!(),
            TypedDecl::Protocol(p) => {
                let _ = self.preload_protocol(p)?;
            }
            TypedDecl::Extension(e) => {
                let _ = self.preload_extension(e)?;
            }
        }
        Result::Ok(())
    }

    pub fn preload_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        let c_name_space = self.context.current_namespace.clone();
        self.context.push_local_stack();
        let arg_defs = f
            .arg_defs
            .into_iter()
            .map(|a| {
                let a = self.typed_arg_def(a)?;
                self.context
                    .register_to_env(a.name.clone(), a.type_.clone());
                Result::Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&f.name, &f.return_type, &f.body)?;
        let fun = TypedFun {
            annotations: f.annotations,
            package: TypedPackage::Resolved(Package::from(c_name_space)),
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params,
            type_constraints: f.type_constraints,
            arg_defs,
            body: None,
            return_type: Some(return_type),
        };
        self.context.pop_local_stack();
        Result::Ok(fun)
    }

    pub fn preload_struct(&mut self, s: TypedStruct) -> Result<()> {
        let TypedStruct {
            annotations: _,
            package: _,
            name,
            type_params: _,
            initializers,
            stored_properties,
            computed_properties,
            member_functions,
        } = s;
        let current_namespace = self.context.current_namespace.clone();
        let this_type = TypedType::Value(TypedValueType::Value(TypedNamedValueType {
            package: TypedPackage::Resolved(Package::from(current_namespace)),
            name: name.clone(),
            type_args: None,
        }));
        self.context.set_current_type(this_type.clone());
        for stored_property in stored_properties.into_iter() {
            let type_ = self.context.full_type_name(stored_property.type_)?;
            let ns = self.context.get_current_namespace_mut()?;
            let rs = ns.get_type_mut(&name).ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.stored_properties.insert(stored_property.name, type_);
        }
        for computed_property in computed_properties.into_iter() {
            let type_ = self.context.full_type_name(computed_property.type_)?;
            let ns = self.context.get_current_namespace_mut()?;
            let rs = ns.get_type_mut(&name).ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.computed_properties.insert(computed_property.name, type_);
        }
        for member_function in member_functions.into_iter() {
            let type_ = self
                .context
                .full_type_name(member_function.type_().unwrap())?;
            let ns = self.context.get_current_namespace_mut()?;
            let rs = ns.get_type_mut(&name).ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.member_functions.insert(member_function.name, type_);
        }
        for ini in initializers.iter() {
            let type_ =
                self.context
                    .full_type_name(TypedType::Function(Box::new(TypedFunctionType {
                        arguments: ini.args.iter().map(|a| a.to_arg_type()).collect(),
                        return_type: this_type.clone(),
                    })))?;
            let ns = self.context.get_current_namespace_mut()?;
            let rs = ns.get_type_mut(&name).ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.static_functions.insert(String::from("init"), type_);
        }
        self.context.clear_current_type();

        Result::Ok(())
    }

    pub fn preload_extension(&mut self, e: TypedExtension) -> Result<()> {
        let TypedExtension {
            annotations,
            name,
            protocol: type_params,
            computed_properties,
            member_functions,
        } = e;
        let this_type = self.context.full_type_name(name)?;
        self.context.set_current_type(this_type.clone());
        for computed_property in computed_properties {
            let type_ = self.context.full_type_name(computed_property.type_)?;
            let ns = self
                .context
                .get_namespace_mut(this_type.package().into_resolved().names)?;
            let rs = ns.get_type_mut(&this_type.name()).ok_or_else(|| {
                ResolverError::from(format!(
                    "Struct {:?} not exist. Maybe before preload",
                    this_type
                ))
            })?;
            rs.computed_properties.insert(computed_property.name, type_);
        }
        for member_function in member_functions {
            let type_ = self
                .context
                .full_type_name(member_function.type_().unwrap())?;
            let ns = self
                .context
                .get_namespace_mut(this_type.package().into_resolved().names)?;
            let rs = ns.get_type_mut(&this_type.name()).ok_or_else(|| {
                ResolverError::from(format!(
                    "Struct {:?} not exist. Maybe before preload",
                    this_type
                ))
            })?;
            rs.member_functions.insert(member_function.name, type_);
        }
        self.context.clear_current_type();
        Ok(())
    }

    pub fn preload_protocol(&mut self, p: TypedProtocol) -> Result<()> {
        Ok(())
    }

    pub fn source_set(&mut self, s: TypedSourceSet) -> Result<TypedSourceSet> {
        Result::Ok(match s {
            TypedSourceSet::File(f) => TypedSourceSet::File(self.file(f)?),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name.clone());
                let items = items
                    .into_iter()
                    .map(|i| self.source_set(i))
                    .collect::<Result<Vec<TypedSourceSet>>>()?;
                self.context.pop_name_space();
                TypedSourceSet::Dir { name, items }
            }
        })
    }

    pub fn file(&mut self, f: TypedFile) -> Result<TypedFile> {
        self.context.push_name_space(f.name.clone());
        for u in f.uses.iter() {
            self.context.use_name_space(u.package.names.clone());
        }
        let result = Result::Ok(TypedFile {
            name: f.name,
            uses: vec![],
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<Result<Vec<TypedDecl>>>()?,
        });
        for u in f.uses.into_iter() {
            self.context.unuse_name_space(u.package.names);
        }
        self.context.pop_name_space();
        result
    }

    pub fn decl(&mut self, d: TypedDecl) -> Result<TypedDecl> {
        Result::Ok(match d {
            TypedDecl::Var(v) => TypedDecl::Var(self.typed_var(v)?),
            TypedDecl::Fun(f) => TypedDecl::Fun(self.typed_fun(f)?),
            TypedDecl::Struct(s) => TypedDecl::Struct(self.typed_struct(s)?),
            TypedDecl::Class => TypedDecl::Class,
            TypedDecl::Enum => TypedDecl::Enum,
            TypedDecl::Protocol(p) => TypedDecl::Protocol(p),
            TypedDecl::Extension(e) => TypedDecl::Extension(self.typed_extension(e)?),
        })
    }

    pub fn typed_var(&mut self, t: TypedVar) -> Result<TypedVar> {
        let TypedVar {
            annotations,
            package,
            is_mut,
            name,
            type_,
            value,
        } = t;
        let value = self.expr(
            value,
            match type_ {
                Some(type_) => Some(self.context.full_type_name(type_)?),
                None => None,
            },
        )?;
        let v = TypedVar {
            annotations,
            package: TypedPackage::Resolved(Package::new()),
            is_mut,
            name,
            type_: value.type_(),
            value,
        };
        self.context.register_to_env(
            v.name.clone(),
            v.type_
                .clone()
                .ok_or_else(|| ResolverError::from("Cannot resolve variable type"))?,
        );
        Result::Ok(v)
    }

    fn typed_function_return_type(
        &mut self,
        name: &str,
        return_type: &Option<TypedType>,
        body: &Option<TypedFunBody>,
    ) -> Result<TypedType> {
        match return_type {
            None => match body {
                None => Result::Err(ResolverError::from(format!(
                    "abstract function {:?} must be define type",
                    name
                ))),
                Some(TypedFunBody::Block(_)) => Result::Ok(TypedType::unit()),
                Some(TypedFunBody::Expr(e)) => {
                    self.expr(e.clone(), None)?.type_().ok_or_else(|| {
                        ResolverError::from(format!(
                            "Can not resolve expr type at function {:?}",
                            name
                        ))
                    })
                }
            },
            Some(b) => self.context.full_type_name(b.clone()),
        }
    }

    fn typed_arg_def(&mut self, a: TypedArgDef) -> Result<TypedArgDef> {
        Result::Ok(TypedArgDef {
            label: a.label,
            name: a.name,
            type_: self.context.full_type_name(a.type_)?,
        })
    }

    pub fn typed_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        let c_name_space = self.context.current_namespace.clone();
        self.context.push_local_stack();
        let arg_defs = f
            .arg_defs
            .into_iter()
            .map(|a| {
                let a = self.typed_arg_def(a)?;
                self.context
                    .register_to_env(a.name.clone(), a.type_.clone());
                Result::Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&f.name, &f.return_type, &f.body)?;
        let fun = TypedFun {
            annotations: f.annotations,
            package: TypedPackage::Resolved(Package::from(c_name_space)),
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params,
            type_constraints: f.type_constraints,
            arg_defs,
            body: match f.body {
                Some(b) => Some(self.typed_fun_body(b)?),
                None => None,
            },
            return_type: Some(return_type),
        };
        let fun_name = fun.name.clone();
        let fun_type = fun.type_();
        let result = Result::Ok(fun);
        self.context.pop_local_stack();
        let ns = self.context.get_current_namespace_mut()?;
        ns.register_value(fun_name, fun_type.unwrap());
        result
    }

    pub fn typed_struct(&mut self, s: TypedStruct) -> Result<TypedStruct> {
        let TypedStruct {
            annotations,
            package: _,
            name,
            type_params,
            initializers,
            stored_properties,
            computed_properties, // TODO
            member_functions,
        } = s;
        let current_namespace = self.context.current_namespace.clone();
        let this_type = TypedType::Value(TypedValueType::Value(TypedNamedValueType {
            package: TypedPackage::Resolved(Package::from(current_namespace)),
            name: name.clone(),
            type_args: None,
        }));
        self.context.set_current_type(this_type);
        let initializers = initializers
            .into_iter()
            .map(|i| self.typed_initializer(i))
            .collect::<Result<Vec<_>>>()?;
        let stored_properties = stored_properties
            .into_iter()
            .map(|s| self.typed_stored_property(s))
            .collect::<Result<Vec<_>>>()?;
        let computed_properties = computed_properties.into_iter().collect();
        let member_functions = member_functions
            .into_iter()
            .map(|m| self.typed_member_function(m))
            .collect::<Result<Vec<_>>>()?;
        self.context.clear_current_type();
        Result::Ok(TypedStruct {
            annotations,
            package: TypedPackage::Resolved(Package::from(self.context.current_namespace.clone())),
            name,
            type_params,
            initializers,
            stored_properties,
            computed_properties,
            member_functions,
        })
    }

    fn typed_initializer(&mut self, i: TypedInitializer) -> Result<TypedInitializer> {
        self.context.push_local_stack();

        let self_type = self.context.resolve_current_type()?;
        self.context.register_to_env("self".to_string(), self_type);

        let result = TypedInitializer {
            args: i
                .args
                .into_iter()
                .map(|a| {
                    let a = self.typed_arg_def(a)?;
                    self.context
                        .register_to_env(a.name.clone(), a.type_.clone());
                    Result::Ok(a)
                })
                .collect::<Result<Vec<_>>>()?,
            body: self.typed_fun_body(i.body)?,
        };
        self.context.pop_local_stack();
        Result::Ok(result)
    }

    fn typed_stored_property(&mut self, s: TypedStoredProperty) -> Result<TypedStoredProperty> {
        let TypedStoredProperty { name, type_ } = s;
        Result::Ok(TypedStoredProperty {
            name,
            type_: self.context.full_type_name(type_)?,
        })
    }

    fn typed_member_function(&mut self, mf: TypedMemberFunction) -> Result<TypedMemberFunction> {
        self.context.push_local_stack();
        let arg_defs = mf
            .arg_defs
            .into_iter()
            .map(|a| {
                let a = self.typed_arg_def(a)?;
                self.context
                    .register_to_env(a.name.clone(), a.type_.clone());
                Result::Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&mf.name, &mf.return_type, &mf.body)?;
        let result = Result::Ok(TypedMemberFunction {
            name: mf.name,
            arg_defs,
            type_params: mf.type_params,
            body: match mf.body {
                None => None,
                Some(body) => Some(self.typed_fun_body(body)?),
            },
            return_type: Some(return_type),
        });
        self.context.pop_local_stack();
        result
    }

    fn typed_fun_body(&mut self, b: TypedFunBody) -> Result<TypedFunBody> {
        Result::Ok(match b {
            TypedFunBody::Expr(e) => TypedFunBody::Expr(self.expr(e, None)?),
            TypedFunBody::Block(b) => TypedFunBody::Block(self.typed_block(b)?),
        })
    }

    fn typed_extension(&mut self, e: TypedExtension) -> Result<TypedExtension> {
        let this_type = self.context.full_type_name(e.name)?;
        self.context.set_current_type(this_type.clone());
        let result = Ok(TypedExtension {
            annotations: e.annotations,
            name: this_type,
            protocol: e.protocol, // TODO
            computed_properties: e.computed_properties.into_iter().map(|i| i).collect(),
            member_functions: e
                .member_functions
                .into_iter()
                .map(|m| self.typed_member_function(m))
                .collect::<Result<Vec<_>>>()?,
        });
        self.context.clear_current_type();
        result
    }

    fn typed_block(&mut self, b: TypedBlock) -> Result<TypedBlock> {
        Result::Ok(TypedBlock {
            body: b
                .body
                .into_iter()
                .map(|s| self.stmt(s))
                .collect::<Result<Vec<TypedStmt>>>()?,
        })
    }

    pub fn expr(&mut self, e: TypedExpr, type_annotation: Option<TypedType>) -> Result<TypedExpr> {
        Result::Ok(match e {
            TypedExpr::Name(n) => TypedExpr::Name(self.typed_name(n, type_annotation)?),
            TypedExpr::Literal(l) => TypedExpr::Literal(self.typed_literal(l, type_annotation)?),
            TypedExpr::BinOp(b) => TypedExpr::BinOp(self.typed_binop(b)?),
            TypedExpr::UnaryOp(u) => TypedExpr::UnaryOp(self.typed_unary_op(u)?),
            TypedExpr::Subscript(s) => TypedExpr::Subscript(self.typed_subscript(s)?),
            TypedExpr::Member(m) => TypedExpr::Member(self.typed_instance_member(m)?),
            TypedExpr::Array(a) => TypedExpr::Array(self.typed_array(a)?),
            TypedExpr::Tuple => TypedExpr::Tuple,
            TypedExpr::Dict => TypedExpr::Dict,
            TypedExpr::StringBuilder => TypedExpr::StringBuilder,
            TypedExpr::Call(c) => TypedExpr::Call(self.typed_call(c)?),
            TypedExpr::If(i) => TypedExpr::If(self.typed_if(i)?),
            TypedExpr::When => TypedExpr::When,
            TypedExpr::Lambda(l) => TypedExpr::Lambda(l),
            TypedExpr::Return(r) => TypedExpr::Return(self.typed_return(r)?),
            TypedExpr::TypeCast(t) => TypedExpr::TypeCast(self.typed_type_cast(t)?),
        })
    }

    pub fn typed_name(
        &mut self,
        n: TypedName,
        type_annotation: Option<TypedType>,
    ) -> Result<TypedName> {
        let (type_, package) = self.context.resolve_name_type(
            n.package.into_raw().names,
            n.name.clone(),
            type_annotation,
        )?;
        Result::Ok(TypedName {
            package,
            type_: Some(type_),
            name: n.name,
        })
    }

    fn typed_literal(
        &mut self,
        l: TypedLiteral,
        type_annotation: Option<TypedType>,
    ) -> Result<TypedLiteral> {
        Ok(match l {
            TypedLiteral::Integer { value, type_ } => TypedLiteral::Integer {
                value,
                type_: if type_.is_some() {
                    type_
                } else if type_annotation.is_some() {
                    type_annotation
                } else {
                    Some(TypedType::int64())
                },
            },
            TypedLiteral::FloatingPoint { value, type_ } => TypedLiteral::FloatingPoint {
                value,
                type_: if type_.is_some() {
                    type_
                } else if type_annotation.is_some() {
                    type_annotation
                } else {
                    Some(TypedType::double())
                },
            },
            TypedLiteral::String { value, type_ } => TypedLiteral::String { value, type_ },
            TypedLiteral::Boolean { value, type_ } => TypedLiteral::Boolean { value, type_ },
            TypedLiteral::NullLiteral { type_: _ } => TypedLiteral::NullLiteral {
                type_: type_annotation,
            },
        })
    }

    pub fn typed_unary_op(&mut self, u: TypedUnaryOp) -> Result<TypedUnaryOp> {
        Result::Ok(match u {
            TypedUnaryOp::Prefix(p) => TypedUnaryOp::Prefix(self.typed_prefix_unary_op(p)?),
            TypedUnaryOp::Postfix(p) => TypedUnaryOp::Postfix(self.typed_postfix_unary_op(p)?),
        })
    }

    pub fn typed_prefix_unary_op(&mut self, u: TypedPrefixUnaryOp) -> Result<TypedPrefixUnaryOp> {
        let target = Box::new(self.expr(*u.target, None)?);
        Result::Ok(match &u.operator {
            TypedPrefixUnaryOperator::Negative
            | TypedPrefixUnaryOperator::Positive
            | TypedPrefixUnaryOperator::Not => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: target.type_(),
                target,
            },
            TypedPrefixUnaryOperator::Reference => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: target
                    .type_()
                    .map(|t| TypedType::Value(TypedValueType::Reference(Box::new(t)))),
                target,
            },
            TypedPrefixUnaryOperator::Dereference => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: match target.type_() {
                    None => None,
                    Some(TypedType::Value(TypedValueType::Reference(t))) => Some(*t),
                    Some(TypedType::Value(TypedValueType::Pointer(t))) => Some(*t),
                    Some(_) => None,
                },
                target,
            },
        })
    }

    pub fn typed_postfix_unary_op(
        &mut self,
        u: TypedPostfixUnaryOp,
    ) -> Result<TypedPostfixUnaryOp> {
        let target = Box::new(self.expr(*u.target, None)?);
        Result::Ok(TypedPostfixUnaryOp {
            operator: u.operator,
            type_: target.type_(),
            target,
        })
    }

    pub fn typed_binop(&mut self, b: TypedBinOp) -> Result<TypedBinOp> {
        let left = self.expr(*b.left, None)?;
        let right = self.expr(*b.right, None)?;
        let (left, right) = match (left, right) {
            (
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: left_value,
                    type_: left_type,
                }),
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: right_value,
                    type_: right_type,
                }),
            ) => (
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: left_value,
                    type_: left_type,
                }),
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: right_value,
                    type_: right_type,
                }),
            ),
            (left, TypedExpr::Literal(TypedLiteral::Integer { value, type_ })) => {
                let left_type = left.type_();
                let is_integer = match &left_type {
                    None => false,
                    Some(t) => t.is_integer(),
                };
                if is_integer {
                    (
                        left,
                        TypedExpr::Literal(TypedLiteral::Integer {
                            value,
                            type_: left_type,
                        }),
                    )
                } else {
                    (
                        left,
                        TypedExpr::Literal(TypedLiteral::Integer { value, type_ }),
                    )
                }
            }
            (left, right) => (left, right),
        };
        let type_ = self.context.resolve_binop_type(
            left.type_().unwrap(),
            b.operator.clone(),
            right.type_().unwrap(),
        )?;
        Result::Ok(TypedBinOp {
            left: Box::new(left),
            operator: b.operator,
            right: Box::new(right),
            type_: Some(type_),
        })
    }

    pub fn typed_instance_member(&mut self, m: TypedInstanceMember) -> Result<TypedInstanceMember> {
        let target = self.expr(*m.target, None)?;
        let type_ = self
            .context
            .resolve_member_type(target.type_().unwrap(), &m.name)?;
        Result::Ok(TypedInstanceMember {
            target: Box::new(target),
            name: m.name,
            is_safe: m.is_safe,
            type_: Some(type_),
        })
    }

    pub fn typed_subscript(&mut self, s: TypedSubscript) -> Result<TypedSubscript> {
        let target = self.expr(*s.target, None)?;
        let target_type = target.type_().unwrap();
        if let TypedType::Value(v) = target_type {
            match v {
                TypedValueType::Value(v) => {
                    if v.is_string() {
                        return Result::Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i, None))
                                .collect::<Result<Vec<_>>>()?,
                            type_: Some(TypedType::uint8()),
                        });
                    }
                }
                TypedValueType::Array(et, _) => {
                    return Result::Ok(TypedSubscript {
                        target: Box::new(target),
                        indexes: s
                            .indexes
                            .into_iter()
                            .map(|i| self.expr(i, None))
                            .collect::<Result<Vec<_>>>()?,
                        type_: Some(*et),
                    })
                }
                TypedValueType::Tuple(_) => {
                    todo!()
                }
                TypedValueType::Pointer(p) => {
                    return Result::Ok(TypedSubscript {
                        target: Box::new(target),
                        indexes: s
                            .indexes
                            .into_iter()
                            .map(|i| self.expr(i, None))
                            .collect::<Result<Vec<_>>>()?,
                        type_: Some(*p),
                    })
                }
                TypedValueType::Reference(r) => {
                    if r.is_string() {
                        return Result::Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i, None))
                                .collect::<Result<Vec<_>>>()?,
                            type_: Some(TypedType::uint8()),
                        });
                    }
                }
            }
        }
        Result::Ok(TypedSubscript {
            target: Box::new(target),
            indexes: s
                .indexes
                .into_iter()
                .map(|i| self.expr(i, None))
                .collect::<Result<Vec<_>>>()?,
            type_: s.type_,
        })
    }

    pub fn typed_array(&mut self, a: TypedArray) -> Result<TypedArray> {
        let elements = a
            .elements
            .into_iter()
            .map(|e| self.expr(e, None))
            .collect::<Result<Vec<_>>>()?;
        let len = elements.len();
        Result::Ok(if let Some(e) = elements.get(0) {
            let e_type = e.type_();
            if elements.iter().all(|e| e.type_() == e_type) {
                TypedArray {
                    elements,
                    type_: e_type
                        .map(|e| TypedType::Value(TypedValueType::Array(Box::new(e), len))),
                }
            } else {
                return Result::Err(ResolverError::from("Array elements must be same type."));
            }
        } else {
            // empty case
            TypedArray {
                elements,
                type_: None,
            }
        })
    }

    pub fn typed_call(&mut self, c: TypedCall) -> Result<TypedCall> {
        let args = c
            .args
            .into_iter()
            .map(|c| self.typed_call_arg(c))
            .collect::<Result<Vec<_>>>()?;
        let arg_annotation = TypedType::Function(Box::new(TypedFunctionType {
            arguments: args
                .iter()
                .map(|a| TypedArgType {
                    label: a.label.clone().unwrap_or("_".to_string()),
                    typ: a.arg.type_().unwrap(),
                })
                .collect(),
            return_type: TypedType::noting(),
        }));
        let target = Box::new(self.expr(*c.target, Some(arg_annotation))?);
        let c_type = match target.type_().unwrap() {
            TypedType::Value(v) => Err(ResolverError::from(format!("{:?} is not callable.", v))),
            TypedType::Type(t) => Err(ResolverError::from(format!("{:?} is not callable.", t))),
            TypedType::Self_ => Err(ResolverError::from("Self is not callable.")),
            TypedType::Function(f) => Ok(f.return_type),
        }?;
        Ok(TypedCall {
            target,
            args,
            type_: Some(c_type),
        })
    }

    pub fn typed_call_arg(&mut self, a: TypedCallArg) -> Result<TypedCallArg> {
        Result::Ok(TypedCallArg {
            label: a.label,
            arg: Box::new(self.expr(*a.arg, None)?),
            is_vararg: a.is_vararg,
        })
    }

    pub fn typed_if(&mut self, i: TypedIf) -> Result<TypedIf> {
        let condition = Box::new(self.expr(*i.condition, None)?);
        let body = self.typed_block(i.body)?;
        let else_body = match i.else_body {
            Some(b) => Some(self.typed_block(b)?),
            None => None,
        };
        let type_ = if let Some(else_body) = &else_body {
            else_body.type_().unwrap_or_else(TypedType::noting)
        } else {
            TypedType::noting()
        };
        Result::Ok(TypedIf {
            condition,
            body,
            else_body,
            type_: Some(type_),
        })
    }

    pub fn typed_return(&mut self, r: TypedReturn) -> Result<TypedReturn> {
        let value = match r.value {
            Some(v) => Some(Box::new(self.expr(*v, None)?)),
            None => None,
        };
        Result::Ok(TypedReturn { value })
    }

    pub fn typed_type_cast(&mut self, t: TypedTypeCast) -> Result<TypedTypeCast> {
        Result::Ok(TypedTypeCast {
            target: Box::new(self.expr(*t.target, None)?),
            is_safe: t.is_safe,
            type_: Some(self.context.full_type_name(t.type_.unwrap())?),
        })
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Result<TypedStmt> {
        Result::Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(self.expr(e, None)?),
            TypedStmt::Decl(d) => TypedStmt::Decl(self.decl(d)?),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(self.assignment_stmt(a)?),
            TypedStmt::Loop(l) => TypedStmt::Loop(self.typed_loop_stmt(l)?),
        })
    }

    pub fn assignment_stmt(&mut self, a: TypedAssignmentStmt) -> Result<TypedAssignmentStmt> {
        Result::Ok(match a {
            TypedAssignmentStmt::Assignment(a) => {
                TypedAssignmentStmt::Assignment(self.typed_assignment(a)?)
            }
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(self.typed_assignment_and_operation(a)?)
            }
        })
    }

    pub fn typed_assignment(&mut self, a: TypedAssignment) -> Result<TypedAssignment> {
        Result::Ok(TypedAssignment {
            target: self.expr(a.target, None)?,
            value: self.expr(a.value, None)?,
        })
    }

    pub fn typed_assignment_and_operation(
        &mut self,
        a: TypedAssignmentAndOperation,
    ) -> Result<TypedAssignmentAndOperation> {
        Result::Ok(TypedAssignmentAndOperation {
            target: self.expr(a.target, None)?,
            operator: a.operator, // TODO
            value: self.expr(a.value, None)?,
        })
    }

    pub fn typed_loop_stmt(&mut self, l: TypedLoopStmt) -> Result<TypedLoopStmt> {
        Result::Ok(match l {
            TypedLoopStmt::While(w) => TypedLoopStmt::While(self.typed_while_loop_stmt(w)?),
            TypedLoopStmt::For(f) => TypedLoopStmt::For(self.typed_for_loop_stmt(f)?),
        })
    }

    pub fn typed_while_loop_stmt(&mut self, w: TypedWhileLoopStmt) -> Result<TypedWhileLoopStmt> {
        let TypedWhileLoopStmt { condition, block } = w;
        let condition = self.expr(condition, None)?;
        if !condition.type_().unwrap().is_boolean() {
            return Result::Err(ResolverError::from("while loop condition must be boolean"));
        };
        Result::Ok(TypedWhileLoopStmt {
            condition,
            block: self.typed_block(block)?,
        })
    }

    pub fn typed_for_loop_stmt(&mut self, f: TypedForStmt) -> Result<TypedForStmt> {
        let TypedForStmt {
            values,
            iterator,
            block,
        } = f;
        Result::Ok(TypedForStmt {
            values,
            iterator: self.expr(iterator, None)?,
            block: self.typed_block(block)?,
        })
    }
}

impl From<ResolverContext> for TypeResolver {
    fn from(context: ResolverContext) -> Self {
        Self { context }
    }
}
