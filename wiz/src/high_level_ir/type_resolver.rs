mod error;

use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::typed_decl::{TypedDecl, TypedFun, TypedVar};
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::TypedStmt;
use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverStruct {
    stored_properties: HashMap<String, TypedType>,
    // initializers: Vec<>,
    computed_properties: HashMap<String, TypedType>,
    member_functions: HashMap<String, TypedType>,
    static_functions: HashMap<String, TypedType>,
    conformed_protocols: HashSet<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>,
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct NameSpace {
    children: HashMap<String, NameSpace>,
    types: HashMap<String, ResolverStruct>,
    structs: HashMap<String, TypedType>,
    functions: HashMap<String, TypedType>,
    values: HashMap<String, TypedType>,
}

impl NameSpace {
    fn new() -> Self {
        Self {
            children: Default::default(),
            types: Default::default(),
            structs: Default::default(),
            functions: Default::default(),
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverSubscript {
    target: TypedType,
    indexes: Vec<TypedType>,
    return_type: TypedType,
}
#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverBinary {
    right: TypedType,
    left: TypedType,
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverContext {
    name_space: NameSpace,
    binary_operators: HashMap<BinaryOperator, Vec<ResolverBinary>>,
    subscripts: Vec<ResolverSubscript>,
    current_namespace: Vec<String>,
}

impl ResolverContext {
    fn new() -> Self {
        Self {
            name_space: NameSpace::new(),
            binary_operators: Default::default(),
            subscripts: vec![],
            current_namespace: vec![],
        }
    }

    pub fn push_name_space(&mut self, name: String) {
        self.current_namespace.push(name);
    }

    pub fn pop_name_space(&mut self) {
        self.current_namespace.pop();
    }

    pub fn get_current_namespace_mut(&mut self) -> Option<&mut NameSpace> {
        self.name_space
            .get_child_mut(self.current_namespace.clone())
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

pub type ResolverResult<T> = Result<T, ResolverError>;

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            context: ResolverContext::new(),
        }
    }

    pub fn detect_type(&mut self, f: TypedFile) -> ResolverResult<()> {
        self.context.push_name_space(f.name);
        for d in f.body {
            match d {
                TypedDecl::Struct(s) => {
                    let ns = self
                        .context
                        .get_current_namespace_mut()
                        .ok_or(ResolverError::from("Context NameSpace Error"))?;
                    ns.types.insert(s.name, ResolverStruct::new());
                }
                TypedDecl::Class => {}
                TypedDecl::Enum => {}
                TypedDecl::Protocol => {}
                _ => {}
            }
        }
        self.context.pop_name_space();
        ResolverResult::Ok(())
    }

    pub fn preload_file(&mut self, f: TypedFile) -> ResolverResult<()> {
        self.context.push_name_space(f.name);
        for d in f.body {
            self.preload_decl(d)?;
        }
        self.context.pop_name_space();
        ResolverResult::Ok(())
    }

    fn preload_decl(&mut self, d: TypedDecl) -> ResolverResult<()> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.typed_var(v)?;
                let mut env = &mut self.context.name_space;
                env.values.insert(
                    v.name,
                    v.type_
                        .ok_or(ResolverError::from("Cannot resolve variable type"))?,
                );
            }
            TypedDecl::Fun(_) => {}
            TypedDecl::Struct(_) => {}
            TypedDecl::Class => {}
            TypedDecl::Enum => {}
            TypedDecl::Protocol => {}
            TypedDecl::Extension => {}
        }
        ResolverResult::Ok(())
    }

    pub fn file(&self, f: TypedFile) -> ResolverResult<TypedFile> {
        ResolverResult::Ok(TypedFile {
            name: f.name,
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<ResolverResult<Vec<TypedDecl>>>()?,
        })
    }

    pub fn decl(&self, d: TypedDecl) -> ResolverResult<TypedDecl> {
        ResolverResult::Ok(match d {
            TypedDecl::Var(v) => TypedDecl::Var(v),
            TypedDecl::Fun(f) => TypedDecl::Fun(f),
            TypedDecl::Struct(s) => TypedDecl::Struct(s),
            TypedDecl::Class => TypedDecl::Class,
            TypedDecl::Enum => TypedDecl::Enum,
            TypedDecl::Protocol => TypedDecl::Protocol,
            TypedDecl::Extension => TypedDecl::Extension,
        })
    }

    pub fn typed_var(&self, t: TypedVar) -> ResolverResult<TypedVar> {
        ResolverResult::Ok(TypedVar {
            is_mut: t.is_mut,
            name: t.name,
            type_: t.type_,
            value: self.expr(t.value)?,
        })
    }

    pub fn expr(&self, e: TypedExpr) -> ResolverResult<TypedExpr> {
        ResolverResult::Ok(match e {
            TypedExpr::Name(n) => TypedExpr::Name(n),
            TypedExpr::Literal(l) => TypedExpr::Literal(l),
            TypedExpr::BinOp(b) => TypedExpr::BinOp(b),
            TypedExpr::UnaryOp(u) => TypedExpr::UnaryOp(u),
            TypedExpr::Subscript(s) => TypedExpr::Subscript(s),
            TypedExpr::Member(m) => TypedExpr::Member(m),
            TypedExpr::StaticMember(s) => TypedExpr::StaticMember(s),
            TypedExpr::List => TypedExpr::List,
            TypedExpr::Tuple => TypedExpr::Tuple,
            TypedExpr::Dict => TypedExpr::Dict,
            TypedExpr::StringBuilder => TypedExpr::StringBuilder,
            TypedExpr::Call(c) => TypedExpr::Call(c),
            TypedExpr::If(i) => TypedExpr::If(i),
            TypedExpr::When => TypedExpr::When,
            TypedExpr::Lambda => TypedExpr::Lambda,
            TypedExpr::Return(r) => TypedExpr::Return(r),
            TypedExpr::TypeCast => TypedExpr::TypeCast,
            TypedExpr::Type(t) => TypedExpr::Type(t),
        })
    }

    pub fn stmt(&self, s: TypedStmt) -> ResolverResult<TypedStmt> {
        ResolverResult::Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(e),
            TypedStmt::Decl(d) => TypedStmt::Decl(d),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(a),
            TypedStmt::Loop(l) => TypedStmt::Loop(l),
        })
    }
}
