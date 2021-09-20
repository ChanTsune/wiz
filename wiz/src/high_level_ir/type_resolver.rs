pub mod context;
pub mod error;
pub mod result;

use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::type_resolver::context::{ResolverContext, ResolverStruct};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_decl::{TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedMemberFunction, TypedStruct, TypedValueArgDef, TypedVar, TypedInitializer};
use crate::high_level_ir::typed_expr::{
    TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember, TypedLiteral,
    TypedName, TypedReturn, TypedSubscript,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedValueType};
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            context: ResolverContext::new(),
        }
    }

    pub fn detect_type(&mut self, f: &TypedFile) -> Result<()> {
        self.context.push_name_space(f.name.clone());
        let current_namespace = self.context.current_namespace.clone();
        let ns = self.context.get_current_namespace_mut()?;
        for d in f.body.iter() {
            match d {
                TypedDecl::Struct(s) => {
                    ns.types.insert(s.name.clone(), ResolverStruct::new());
                    ns.values.insert(
                        s.name.clone(),
                        TypedType::Type(TypedValueType {
                            package: Package {
                                names: current_namespace.clone(),
                            },
                            name: s.name.clone(),
                            type_args: None,
                        }),
                    );
                }
                TypedDecl::Class => {}
                TypedDecl::Enum => {}
                TypedDecl::Protocol => {}
                _ => {}
            }
        }
        self.context.pop_name_space();
        Result::Ok(())
    }

    pub fn preload_file(&mut self, f: TypedFile) -> Result<()> {
        let name = f.name.clone();
        if name != String::from("builtin.ll") {
            self.context.push_name_space(f.name.clone());
        };
        for d in f.body {
            self.preload_decl(d)?;
        }
        if name != String::from("builtin.ll") {
            self.context.pop_name_space();
        };
        Result::Ok(())
    }

    fn preload_decl(&mut self, d: TypedDecl) -> Result<()> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.typed_var(v)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace.values.insert(
                    v.name,
                    v.type_
                        .ok_or(ResolverError::from("Cannot resolve variable type"))?,
                );
            }
            TypedDecl::Fun(f) => {
                let fun = self.preload_fun(f)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace
                    .values
                    .insert(fun.name.clone(), fun.type_().unwrap());
            }
            TypedDecl::Struct(_) => {}
            TypedDecl::Class => {}
            TypedDecl::Enum => {}
            TypedDecl::Protocol => {}
            TypedDecl::Extension => {}
        }
        Result::Ok(())
    }

    pub fn preload_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        let return_type = self.typed_function_return_type(&f)?;
        let fun = TypedFun {
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params, // TODO
            arg_defs: f.arg_defs,
            body: None,
            return_type: Some(return_type),
        };
        Result::Ok(fun)
    }

    pub fn file(&mut self, f: TypedFile) -> Result<TypedFile> {
        let name = f.name.clone();
        if name != String::from("builtin.ll") {
            self.context.push_name_space(f.name.clone());
        };
        let result = Result::Ok(TypedFile {
            name: f.name,
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<Result<Vec<TypedDecl>>>()?,
        });
        if name != String::from("builtin.ll") {
            self.context.pop_name_space();
        };
        result
    }

    pub fn decl(&mut self, d: TypedDecl) -> Result<TypedDecl> {
        Result::Ok(match d {
            TypedDecl::Var(v) => TypedDecl::Var(self.typed_var(v)?),
            TypedDecl::Fun(f) => TypedDecl::Fun(self.typed_fun(f)?),
            TypedDecl::Struct(s) => TypedDecl::Struct(self.typed_struct(s)?),
            TypedDecl::Class => TypedDecl::Class,
            TypedDecl::Enum => TypedDecl::Enum,
            TypedDecl::Protocol => TypedDecl::Protocol,
            TypedDecl::Extension => TypedDecl::Extension,
        })
    }

    pub fn typed_var(&mut self, t: TypedVar) -> Result<TypedVar> {
        let value = self.expr(t.value)?;
        let v_type = match (t.type_, value.type_()) {
            (Some(vt), Some(et)) => {
                if vt != et {
                    Result::Err(ResolverError::from(format!(
                        "Type unmatched {:?} != {:?}",
                        vt, et
                    )))
                } else {
                    Result::Ok(et)
                }
            }
            (Some(vt), None) => {
                eprintln!("maybe invalid type ...");
                Result::Ok(vt)
            }
            (None, Some(et)) => Result::Ok(et),
            (None, None) => Result::Err(ResolverError::from(format!(
                "Can not resolve var type {:?}",
                value
            ))),
        }?;
        let v = TypedVar {
            is_mut: t.is_mut,
            name: t.name,
            type_: Some(v_type),
            value,
        };
        let namespace = self.context.get_current_namespace_mut()?;
        namespace.values.insert(
            v.name.clone(),
            v.type_
                .clone()
                .ok_or(ResolverError::from("Cannot resolve variable type"))?,
        );
        Result::Ok(v)
    }

    fn typed_function_return_type(&mut self, f: &TypedFun) -> Result<TypedType> {
        match &f.return_type {
            None => match &f.body {
                None => Result::Err(ResolverError::from(format!(
                    "abstract function {:?} must be define type",
                    f.name
                ))),
                Some(TypedFunBody::Block(_)) => Result::Ok(TypedType::unit()),
                Some(TypedFunBody::Expr(e)) => {
                    self.expr(e.clone())?
                        .type_()
                        .ok_or(ResolverError::from(format!(
                            "Can not resolve expr type at function {:?}",
                            f.name
                        )))
                }
            },
            Some(b) => self.context.full_type_name(b.clone()),
        }
    }

    fn typed_arg_def(&mut self, a: TypedArgDef) -> Result<TypedArgDef> {
        Result::Ok(match a {
            TypedArgDef::Value(a) => TypedArgDef::Value(TypedValueArgDef {
                label: a.label,
                name: a.name,
                type_: self.context.full_type_name(a.type_)?,
            }),
            TypedArgDef::Self_(s) => TypedArgDef::Self_(s),
        })
    }

    pub fn typed_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        self.context.push_name_space(f.name.clone());
        let return_type = self.typed_function_return_type(&f)?;
        let fun = TypedFun {
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params, // TODO
            arg_defs: f
                .arg_defs
                .into_iter()
                .map(|a| {
                    let a = self.typed_arg_def(a)?;
                    let ns = self.context.get_current_namespace_mut()?;
                    ns.values.insert(
                        a.name(),
                        a.type_()
                            .ok_or(ResolverError::from("Can not resolve 'self type'"))?,
                    );
                    Result::Ok(a)
                })
                .collect::<Result<Vec<TypedArgDef>>>()?,
            body: match f.body {
                Some(b) => Some(self.typed_fun_body(b)?),
                None => None,
            },
            return_type: Some(return_type),
        };
        let fun_name = fun.name.clone();
        let fun_type = fun.type_();
        let result = Result::Ok(fun);
        self.context.pop_name_space();
        let ns = self.context.get_current_namespace_mut()?;
        ns.values.insert(fun_name, fun_type.unwrap());
        result
    }

    pub fn typed_struct(&mut self, s: TypedStruct) -> Result<TypedStruct> {
        let TypedStruct {
            name,
            type_params,
            init,                // TODO
            stored_properties,   // TODO
            computed_properties, // TODO
            member_functions,    // TODO
            static_function,     // TODO
        } = s;
        let current_namespace = self.context.current_namespace.clone();
        let ns = self.context.get_current_namespace_mut()?;
        let this_type = TypedType::Value(TypedValueType {
            package: Package {
                names: current_namespace,
            },
            name: name.clone(),
            type_args: None,
        });
        let rs = ns.types.get_mut(&*name).ok_or(ResolverError::from(format!(
            "Struct {:?} not exist. Maybe before preload",
            name
        )))?;
        for sp in stored_properties.iter() {
            rs.stored_properties
                .insert(sp.name.clone(), sp.type_.clone());
        }
        for cp in computed_properties.iter() {
            rs.computed_properties
                .insert(cp.name.clone(), cp.type_.clone());
        }
        for mf in member_functions.iter() {
            rs.member_functions
                .insert(mf.name.clone(), mf.type_.clone());
        }
        for sf in static_function.iter() {
            rs.static_functions
                .insert(sf.name.clone(), sf.type_().unwrap());
        }
        for ini in init.iter() {
            rs.static_functions.insert(
                String::from("init"),
                TypedType::Function(Box::new(TypedFunctionType {
                    arguments: ini.args.clone(),
                    return_type: this_type.clone(),
                })),
            );
        }
        self.context.set_current_type(this_type);
        self.context.push_name_space(name.clone());
        let init = init.into_iter().map(|i|self.typed_initializer(i)).collect::<Result<Vec<TypedInitializer>>>()?;
        let stored_properties = stored_properties.into_iter().collect();
        let computed_properties = computed_properties.into_iter().collect();
        let member_functions = member_functions
            .into_iter()
            .map(|m| self.typed_member_function(m))
            .collect::<Result<Vec<TypedMemberFunction>>>()?;
        let static_function = static_function.into_iter().collect();
        self.context.clear_current_type();
        self.context.pop_name_space();
        Result::Ok(TypedStruct {
            name,
            type_params,
            init,
            stored_properties,
            computed_properties,
            member_functions,
            static_function,
        })
    }

    fn typed_initializer(&mut self, i: TypedInitializer) -> Result<TypedInitializer> {
        let self_type =                 self.context.get_current_type();
        let ns = self.context.get_current_namespace_mut()?;
        ns.values.insert(
            String::from("self"),
                self_type.ok_or(ResolverError::from("Can not resolve 'self type'"))?,
        );
        Result::Ok(TypedInitializer {
            args: i.args.into_iter()
                .map(|a| {
                    let a = self.typed_arg_def(a)?;
                    let ns = self.context.get_current_namespace_mut()?;
                    ns.values.insert(
                        a.name(),
                        a.type_()
                            .ok_or(ResolverError::from("Can not resolve 'self type'"))?,
                    );
                    Result::Ok(a)
                })
                .collect::<Result<Vec<TypedArgDef>>>()?,
            body: self.typed_fun_body(i.body)?
        })
    }

    fn typed_member_function(&mut self, mf: TypedMemberFunction) -> Result<TypedMemberFunction> {
        self.context.push_name_space(mf.name.clone());
        let self_type = self.context.get_current_type();
        let ns = self.context.get_current_namespace_mut()?;
        let result = Result::Ok(TypedMemberFunction {
            name: mf.name,
            args: mf
                .args
                .into_iter()
                .map(|a| {
                    ns.values.insert(
                        a.name(),
                        a.type_().unwrap_or(
                            self_type
                                .clone()
                                .ok_or(ResolverError::from("Can not resolve `self`"))?,
                        ),
                    );
                    Result::Ok(a)
                })
                .collect::<Result<Vec<TypedArgDef>>>()?,
            type_params: mf.type_params,
            body: self.typed_fun_body(mf.body)?,
            return_type: mf.return_type,
            type_: mf.type_,
        });
        self.context.pop_name_space();
        result
    }

    fn typed_fun_body(&mut self, b: TypedFunBody) -> Result<TypedFunBody> {
        Result::Ok(match b {
            TypedFunBody::Expr(e) => TypedFunBody::Expr(self.expr(e)?),
            TypedFunBody::Block(b) => TypedFunBody::Block(self.typed_block(b)?),
        })
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

    pub fn expr(&mut self, e: TypedExpr) -> Result<TypedExpr> {
        Result::Ok(match e {
            TypedExpr::Name(n) => TypedExpr::Name(self.typed_name(n)?),
            TypedExpr::Literal(l) => TypedExpr::Literal(l),
            TypedExpr::BinOp(b) => TypedExpr::BinOp(self.typed_binop(b)?),
            TypedExpr::UnaryOp(u) => TypedExpr::UnaryOp(u),
            TypedExpr::Subscript(s) => TypedExpr::Subscript(self.typed_subscript(s)?),
            TypedExpr::Member(m) => TypedExpr::Member(self.typed_instance_member(m)?),
            TypedExpr::StaticMember(s) => TypedExpr::StaticMember(s),
            TypedExpr::List => TypedExpr::List,
            TypedExpr::Tuple => TypedExpr::Tuple,
            TypedExpr::Dict => TypedExpr::Dict,
            TypedExpr::StringBuilder => TypedExpr::StringBuilder,
            TypedExpr::Call(c) => TypedExpr::Call(self.typed_call(c)?),
            TypedExpr::If(i) => TypedExpr::If(self.typed_if(i)?),
            TypedExpr::When => TypedExpr::When,
            TypedExpr::Lambda => TypedExpr::Lambda,
            TypedExpr::Return(r) => TypedExpr::Return(self.typed_return(r)?),
            TypedExpr::TypeCast => TypedExpr::TypeCast,
        })
    }

    pub fn typed_name(&mut self, n: TypedName) -> Result<TypedName> {
        Result::Ok(TypedName {
            type_: Some(self.context.resolve_name_type(n.name.clone())?),
            name: n.name,
        })
    }

    pub fn typed_binop(&mut self, b: TypedBinOp) -> Result<TypedBinOp> {
        let left = self.expr(*b.left)?;
        let right = self.expr(*b.right)?;
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
            &*b.kind,
            right.type_().unwrap(),
        )?;
        Result::Ok(TypedBinOp {
            left: Box::new(left),
            kind: b.kind,
            right: Box::new(right),
            type_: Some(type_),
        })
    }

    pub fn typed_instance_member(&mut self, m: TypedInstanceMember) -> Result<TypedInstanceMember> {
        let target = self.expr(*m.target)?;
        let type_ = self
            .context
            .resolve_member_type(target.type_().unwrap(), m.name.clone())?;
        Result::Ok(TypedInstanceMember {
            target: Box::new(target),
            name: m.name,
            is_safe: m.is_safe,
            type_: Some(type_),
        })
    }

    pub fn typed_subscript(&mut self, s: TypedSubscript) -> Result<TypedSubscript> {
        let target = self.expr(*s.target)?;
        if let TypedType::Value(v) = target.type_().unwrap() {
            if v.name == UNSAFE_POINTER {
                if let Some(mut ags) = v.type_args {
                    if ags.len() == 1 {
                        return Result::Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i))
                                .collect::<Result<Vec<TypedExpr>>>()?,
                            type_: Some(ags.remove(0)),
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
                .map(|i| self.expr(i))
                .collect::<Result<Vec<TypedExpr>>>()?,
            type_: s.type_,
        })
    }

    pub fn typed_call(&mut self, c: TypedCall) -> Result<TypedCall> {
        let target = Box::new(self.expr(*c.target)?);
        let c_type = match target.type_().unwrap() {
            TypedType::Value(v) | TypedType::Type(v) => {
                Result::Err(ResolverError::from(format!("{:?} is not callable.", v)))
            }
            TypedType::Function(f) => Result::Ok(f.return_type),
        }?;
        Result::Ok(TypedCall {
            target,
            args: c
                .args
                .into_iter()
                .map(|c| self.typed_call_arg(c))
                .collect::<Result<Vec<TypedCallArg>>>()?,
            type_: Some(c_type),
        })
    }

    pub fn typed_call_arg(&mut self, a: TypedCallArg) -> Result<TypedCallArg> {
        Result::Ok(TypedCallArg {
            label: a.label,
            arg: Box::new(self.expr(*a.arg)?),
            is_vararg: a.is_vararg,
        })
    }

    pub fn typed_if(&mut self, i: TypedIf) -> Result<TypedIf> {
        let condition = Box::new(self.expr(*i.condition)?);
        let body = self.typed_block(i.body)?;
        let else_body = match i.else_body {
            Some(b) => Some(self.typed_block(b)?),
            None => None,
        };
        let type_ = i.type_;
        Result::Ok(TypedIf {
            condition,
            body,
            else_body,
            type_,
        })
    }

    pub fn typed_return(&mut self, r: TypedReturn) -> Result<TypedReturn> {
        let value = match r.value {
            Some(v) => Some(Box::new(self.expr(*v)?)),
            None => None,
        };
        Result::Ok(TypedReturn {
            type_: match &value {
                Some(v) => v.type_(),
                None => None,
            },
            value: value,
        })
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Result<TypedStmt> {
        Result::Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(self.expr(e)?),
            TypedStmt::Decl(d) => TypedStmt::Decl(self.decl(d)?),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(self.assignment_stmt(a)?),
            TypedStmt::Loop(l) => TypedStmt::Loop(l),
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
            target: self.expr(a.target)?,
            value: self.expr(a.value)?,
        })
    }

    pub fn typed_assignment_and_operation(
        &mut self,
        a: TypedAssignmentAndOperation,
    ) -> Result<TypedAssignmentAndOperation> {
        Result::Ok(TypedAssignmentAndOperation {
            target: self.expr(a.target)?,
            operator: a.operator, // TODO
            value: self.expr(a.value)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::UNSAFE_POINTER;
    use crate::high_level_ir::type_resolver::TypeResolver;
    use crate::high_level_ir::typed_decl::{
        TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedInitializer, TypedStoredProperty,
        TypedStruct, TypedValueArgDef, TypedVar,
    };
    use crate::high_level_ir::typed_expr::{
        TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedInstanceMember, TypedLiteral,
        TypedName, TypedReturn,
    };
    use crate::high_level_ir::typed_file::TypedFile;
    use crate::high_level_ir::typed_stmt::{TypedBlock, TypedStmt, TypedAssignmentStmt, TypedAssignment};
    use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedValueType};
    use crate::high_level_ir::Ast2HLIR;
    use crate::parser::parser::parse_from_string;

    #[test]
    fn test_empty() {
        let source = "";

        let ast = parse_from_string(String::from(source)).unwrap();

        let mut ast2hlir = Ast2HLIR::new();

        let mut file = ast2hlir.file(ast);
        file.name = String::from("test");

        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![]
            })
        );
    }

    #[test]
    fn test_unsafe_pointer() {
        let source = r"
        struct A {
            val a: UnsafePointer<UInt8>
        }
        fun function(_ a: A): Unit {
            val a = a.a
        }
        ";

        let ast = parse_from_string(String::from(source)).unwrap();

        let mut ast2hlir = Ast2HLIR::new();

        let mut file = ast2hlir.file(ast);
        file.name = String::from("test");

        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![
                    TypedDecl::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        init: vec![TypedInitializer {
                            args: vec![TypedArgDef::Value(TypedValueArgDef {
                                label: "a".to_string(),
                                name: "a".to_string(),
                                type_: TypedType::Value(TypedValueType {
                                    package: Package::global(),
                                    name: String::from(UNSAFE_POINTER),
                                    type_args: Some(vec![TypedType::uint8()])
                                })
                            })],
                            body: TypedFunBody::Block(TypedBlock { body: vec![
                                TypedStmt::Assignment(TypedAssignmentStmt::Assignment(TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedValueType {
                                                package: Package {
                                                    names: vec![String::from("test")]
                                                },
                                                name: "A".to_string(),
                                                type_args: None
                                            }))
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::Value(TypedValueType {
                                            package: Package::global(),
                                            name: String::from(UNSAFE_POINTER),
                                            type_args: Some(vec![TypedType::uint8()])
                                        }))
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        name: "a".to_string(),
                                        type_: Some(TypedType::Value(TypedValueType {
                                            package: Package::global(),
                                            name: String::from(UNSAFE_POINTER),
                                            type_args: Some(vec![TypedType::uint8()])
                                        }))
                                    })
                                }))
                            ] })
                        }],
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType {
                                package: Package::global(),
                                name: String::from(UNSAFE_POINTER),
                                type_args: Some(vec![TypedType::uint8()])
                            })
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                        static_function: vec![]
                    }),
                    TypedDecl::Fun(TypedFun {
                        modifiers: vec![],
                        name: "function".to_string(),
                        type_params: None,
                        arg_defs: vec![TypedArgDef::Value(TypedValueArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType {
                                package: Package {
                                    names: vec![String::from("test")]
                                },
                                name: "A".to_string(),
                                type_args: None
                            })
                        })],
                        body: Option::Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                                is_mut: false,
                                name: "a".to_string(),
                                type_: Some(TypedType::Value(TypedValueType {
                                    package: Package::global(),
                                    name: String::from(UNSAFE_POINTER),
                                    type_args: Some(vec![TypedType::uint8()])
                                })),
                                value: TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        name: "a".to_string(),
                                        type_: Some(TypedType::Value(TypedValueType {
                                            package: Package {
                                                names: vec![String::from("test")]
                                            },
                                            name: "A".to_string(),
                                            type_args: None
                                        }))
                                    })),
                                    name: "a".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::Value(TypedValueType {
                                        package: Package::global(),
                                        name: String::from(UNSAFE_POINTER),
                                        type_args: Some(vec![TypedType::uint8()])
                                    }))
                                })
                            }))]
                        })),
                        return_type: Some(TypedType::unit())
                    })
                ],
            })
        );
    }

    #[test]
    fn test_struct_member() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    name: "A".to_string(),
                    type_params: None,
                    init: vec![TypedInitializer {
                        args: vec![],
                        body: TypedFunBody::Block(TypedBlock { body: vec![] }),
                    }],
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64(),
                    }],
                    computed_properties: vec![],
                    member_functions: vec![],
                    static_function: vec![],
                }),
                TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef::Value(TypedValueArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedValueType {
                            package: Package {
                                names: vec![String::from("test")],
                            },
                            name: "A".to_string(),
                            type_args: None,
                        }),
                    })],
                    body: Option::Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                            is_mut: false,
                            name: "a".to_string(),
                            type_: None,
                            value: TypedExpr::Member(TypedInstanceMember {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    name: "a".to_string(),
                                    type_: None,
                                })),
                                name: "a".to_string(),
                                is_safe: false,
                                type_: None,
                            }),
                        }))],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            ],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![
                    TypedDecl::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        init: vec![TypedInitializer {
                            args: vec![],
                            body: TypedFunBody::Block(TypedBlock { body: vec![] })
                        }],
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64()
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                        static_function: vec![]
                    }),
                    TypedDecl::Fun(TypedFun {
                        modifiers: vec![],
                        name: "function".to_string(),
                        type_params: None,
                        arg_defs: vec![TypedArgDef::Value(TypedValueArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType {
                                package: Package {
                                    names: vec![String::from("test")]
                                },
                                name: "A".to_string(),
                                type_args: None
                            })
                        })],
                        body: Option::Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                                is_mut: false,
                                name: "a".to_string(),
                                type_: Some(TypedType::int64()),
                                value: TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        name: "a".to_string(),
                                        type_: Some(TypedType::Value(TypedValueType {
                                            package: Package {
                                                names: vec![String::from("test")]
                                            },
                                            name: "A".to_string(),
                                            type_args: None
                                        }))
                                    })),
                                    name: "a".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::int64())
                                })
                            }))]
                        })),
                        return_type: Some(TypedType::unit())
                    })
                ],
            })
        );
    }

    #[test]
    fn test_struct_init() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    name: "A".to_string(),
                    type_params: None,
                    init: vec![TypedInitializer {
                        args: vec![TypedArgDef::Value(TypedValueArgDef {
                            label: "a".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        })],
                        body: TypedFunBody::Block(TypedBlock { body: vec![] }),
                    }],
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64(),
                    }],
                    computed_properties: vec![],
                    member_functions: vec![],
                    static_function: vec![],
                }),
                TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef::Value(TypedValueArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedValueType {
                            package: Package {
                                names: vec![String::from("test")],
                            },
                            name: "A".to_string(),
                            type_args: None,
                        }),
                    })],
                    body: Option::Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                            is_mut: false,
                            name: "a".to_string(),
                            type_: None,
                            value: TypedExpr::Call(TypedCall {
                                target: Box::new(TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        name: "A".to_string(),
                                        type_: None,
                                    })),
                                    name: "init".to_string(),
                                    is_safe: false,
                                    type_: None,
                                })),
                                args: vec![TypedCallArg {
                                    label: Some(String::from("a")),
                                    arg: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                        value: "a".to_string(),
                                        type_: Some(TypedType::int64()),
                                    })),
                                    is_vararg: false,
                                }],
                                type_: None,
                            }),
                        }))],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            ],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![
                    TypedDecl::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        init: vec![TypedInitializer {
                            args: vec![TypedArgDef::Value(TypedValueArgDef {
                                label: "a".to_string(),
                                name: "a".to_string(),
                                type_: TypedType::int64()
                            })],
                            body: TypedFunBody::Block(TypedBlock { body: vec![] }),
                        }],
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                        static_function: vec![],
                    }),
                    TypedDecl::Fun(TypedFun {
                        modifiers: vec![],
                        name: "function".to_string(),
                        type_params: None,
                        arg_defs: vec![TypedArgDef::Value(TypedValueArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType {
                                package: Package {
                                    names: vec![String::from("test")],
                                },
                                name: "A".to_string(),
                                type_args: None,
                            }),
                        })],
                        body: Option::Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                                is_mut: false,
                                name: "a".to_string(),
                                type_: Some(TypedType::Value(TypedValueType {
                                    package: Package {
                                        names: vec![String::from("test")]
                                    },
                                    name: "A".to_string(),
                                    type_args: None
                                })),
                                value: TypedExpr::Call(TypedCall {
                                    target: Box::new(TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            name: "A".to_string(),
                                            type_: Some(TypedType::Type(TypedValueType {
                                                package: Package {
                                                    names: vec![String::from("test")]
                                                },
                                                name: "A".to_string(),
                                                type_args: None
                                            })),
                                        })),
                                        name: "init".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::Function(Box::new(
                                            TypedFunctionType {
                                                arguments: vec![TypedArgDef::Value(
                                                    TypedValueArgDef {
                                                        label: "a".to_string(),
                                                        name: "a".to_string(),
                                                        type_: TypedType::int64()
                                                    }
                                                )],
                                                return_type: TypedType::Value(TypedValueType {
                                                    package: Package {
                                                        names: vec![String::from("test")]
                                                    },
                                                    name: "A".to_string(),
                                                    type_args: None
                                                })
                                            }
                                        ))),
                                    })),
                                    args: vec![TypedCallArg {
                                        label: Some(String::from("a")),
                                        arg: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                            value: "a".to_string(),
                                            type_: Some(TypedType::int64())
                                        })),
                                        is_vararg: false
                                    }],
                                    type_: Some(TypedType::Value(TypedValueType {
                                        package: Package {
                                            names: vec![String::from("test")]
                                        },
                                        name: "A".to_string(),
                                        type_args: None
                                    }))
                                }),
                            }))],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                ],
            })
        );
    }

    #[test]
    fn test_expr_function() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![TypedDecl::Fun(TypedFun {
                modifiers: vec![],
                name: "function".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                    TypedLiteral::Integer {
                        value: "1".to_string(),
                        type_: Some(TypedType::int64()),
                    },
                ))),
                return_type: None,
            })],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                        TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64())
                        }
                    ))),
                    return_type: Some(TypedType::int64())
                })],
            })
        );
    }

    #[test]
    fn test_function_call() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![
                TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "target_function".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                        TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        },
                    ))),
                    return_type: None,
                }),
                TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "main".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::Call(TypedCall {
                            target: Box::new(TypedExpr::Name(TypedName {
                                name: "target_function".to_string(),
                                type_: None,
                            })),
                            args: vec![],
                            type_: None,
                        }))],
                    })),
                    return_type: None,
                }),
            ],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![
                    TypedDecl::Fun(TypedFun {
                        modifiers: vec![],
                        name: "target_function".to_string(),
                        type_params: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                            TypedLiteral::Integer {
                                value: "1".to_string(),
                                type_: Some(TypedType::int64()),
                            },
                        ))),
                        return_type: Some(TypedType::int64()),
                    }),
                    TypedDecl::Fun(TypedFun {
                        modifiers: vec![],
                        name: "main".to_string(),
                        type_params: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExpr::Call(TypedCall {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    name: "target_function".to_string(),
                                    type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                        arguments: vec![],
                                        return_type: TypedType::int64()
                                    })))
                                })),
                                args: vec![],
                                type_: Some(TypedType::int64())
                            }))]
                        })),
                        return_type: Some(TypedType::unit())
                    })
                ],
            })
        );
    }

    #[test]
    fn test_return_integer_literal() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![TypedDecl::Fun(TypedFun {
                modifiers: vec![],
                name: "sample".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                        value: Option::Some(Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        }))),
                        type_: None,
                    }))],
                })),
                return_type: Some(TypedType::int64()),
            })],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "sample".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                            value: Option::Some(Box::new(TypedExpr::Literal(
                                TypedLiteral::Integer {
                                    value: "1".to_string(),
                                    type_: Some(TypedType::int64())
                                }
                            ))),
                            type_: Some(TypedType::int64())
                        }))]
                    })),
                    return_type: Some(TypedType::int64())
                })]
            })
        );
    }

    #[test]
    fn test_binop() {
        let file = TypedFile {
            name: "test".to_string(),
            body: vec![TypedDecl::Fun(TypedFun {
                modifiers: vec![],
                name: "sample".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::BinOp(TypedBinOp {
                        left: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        })),
                        kind: "+".to_string(),
                        type_: None,
                        right: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "2".to_string(),
                            type_: Some(TypedType::int64()),
                        })),
                    }))],
                })),
                return_type: Some(TypedType::unit()),
            })],
        };
        let mut resolver = TypeResolver::new();
        let _ = resolver.detect_type(&file);
        let _ = resolver.preload_file(file.clone());
        let f = resolver.file(file);

        assert_eq!(
            f,
            Result::Ok(TypedFile {
                name: "test".to_string(),
                body: vec![TypedDecl::Fun(TypedFun {
                    modifiers: vec![],
                    name: "sample".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::BinOp(TypedBinOp {
                            left: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                value: "1".to_string(),
                                type_: Some(TypedType::int64()),
                            })),
                            kind: "+".to_string(),
                            right: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                value: "2".to_string(),
                                type_: Some(TypedType::int64()),
                            })),
                            type_: Some(TypedType::int64()),
                        }))],
                    })),
                    return_type: Some(TypedType::unit())
                })]
            })
        );
    }
}
