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
            type_params: None
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
            values: Default::default()
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
            current_namespace: vec![]
        }
    }

    fn push_name_space(&mut self, name: String) {
        self.current_namespace.push(name);
    }

    fn pop_name_space(&mut self) {
        self.current_namespace.pop();
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverError {
    message: String,
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ResolverError: ")?;
        f.write_str(&self.message)?;
        f.write_str("\n")
    }
}

impl Error for ResolverError {}

pub type ResolverResult<T> = Result<T, ResolverError>;

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            context: ResolverContext::new(),
        }
    }

    pub fn preload_file(&mut self, f: TypedFile) {
        self.context.current_namespace.push(f.name);
        for d in f.body {
            self.preload_decl(d);
        }
        self.context.current_namespace.pop();
    }

    fn preload_decl(&mut self, d: TypedDecl) {
        let mut env = &mut self.context.name_space;
        match d {
            TypedDecl::Var(v) => {
                // env.values.insert(name, t);
            }
            TypedDecl::Fun(_) => {}
            TypedDecl::Struct(_) => {}
            TypedDecl::Class => {}
            TypedDecl::Enum => {}
            TypedDecl::Protocol => {}
            TypedDecl::Extension => {}
        }
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
            value: self.expr(t.value)?
        })
    }

    pub fn expr(&self, e: TypedExpr) -> ResolverResult<TypedExpr> {
        ResolverResult::Ok(match e {
            TypedExpr::Name(n) => {TypedExpr::Name(n)}
            TypedExpr::Literal(l) => {TypedExpr::Literal(l)}
            TypedExpr::BinOp(b) => {TypedExpr::BinOp(b)}
            TypedExpr::UnaryOp(u) => {TypedExpr::UnaryOp(u)}
            TypedExpr::Subscript(s) => {TypedExpr::Subscript(s)}
            TypedExpr::Member(m) => {TypedExpr::Member(m)}
            TypedExpr::StaticMember(s) => {TypedExpr::StaticMember(s)}
            TypedExpr::List => {TypedExpr::List}
            TypedExpr::Tuple => {TypedExpr::Tuple }
            TypedExpr::Dict => {TypedExpr::Dict}
            TypedExpr::StringBuilder => {TypedExpr::StringBuilder}
            TypedExpr::Call(c) => {TypedExpr::Call(c)}
            TypedExpr::If(i) => {TypedExpr::If(i)}
            TypedExpr::When => {TypedExpr::When }
            TypedExpr::Lambda => {TypedExpr::Lambda }
            TypedExpr::Return(r) => {TypedExpr::Return(r)}
            TypedExpr::TypeCast => {TypedExpr::TypeCast }
            TypedExpr::Type(t) => {TypedExpr::Type(t)}
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
