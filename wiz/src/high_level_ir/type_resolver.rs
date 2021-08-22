use std::collections::{HashMap, HashSet};
use crate::high_level_ir::typed_decl::{TypedFun, TypedDecl};
use crate::high_level_ir::typed_type::TypedType;
use crate::high_level_ir::typed_file::TypedFile;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::high_level_ir::typed_stmt::TypedStmt;
use crate::high_level_ir::typed_expr::TypedExpr;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverTypeParam {
    type_constraints: Vec<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverStruct {
    stored_properties: HashMap<String,TypedType>,
    // initializers: Vec<>,
    computed_properties: HashMap<String, TypedType>,
    member_functions: HashMap<String, TypedType>,
    static_functions: HashMap<String, TypedType>,
    conformed_protocols: HashSet<String>,
    type_params: Option<HashMap<String, ResolverTypeParam>>
}

impl ResolverStruct {
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
enum ResolverOperators {
    Subscript{
        target: TypedType,
        indexes: Vec<TypedType>,
        return_type: TypedType
    },
    Binary {
        right: TypedType,
        left: TypedType,
        return_type: TypedType
    },
    Unary {
        value: TypedType,
        return_type: TypedType
    },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
enum ResolverOperator {
    Subscript,
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryMod,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
struct ResolverContext {
    name_space: HashMap<String, NameSpace>,
    operators: HashMap<ResolverOperator, Vec<ResolverOperators>>,
}

impl ResolverContext {
    fn new() -> Self {
        Self { name_space: Default::default(), operators: Default::default() }
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ResolverError {
    message: String
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ResolverError: ")?;
        f.write_str(&self.message)?;
        f.write_str("\n")
    }
}

impl Error for ResolverError { }

pub type ResolverResult<T> = Result<T, ResolverError>;

impl TypeResolver {

    pub fn new() -> Self {
        Self { context: ResolverContext::new() }
    }

    pub fn preload(&self, f: TypedFile) {

    }

    pub fn file(&self, f: TypedFile) -> ResolverResult<TypedFile> {
        ResolverResult::Ok(TypedFile {
            name: f.name,
            body: f.body.into_iter().map(|s| {
                self.decl(s)
            }).collect::<ResolverResult<Vec<TypedDecl>>>()?
        })
    }

    pub fn decl(&self, d: TypedDecl) -> ResolverResult<TypedDecl> {
        ResolverResult::Ok(match d {
            TypedDecl::Var(v) => {
                TypedDecl::Var(v)
            }
            TypedDecl::Fun(f) => {
                TypedDecl::Fun(f)
            }
            TypedDecl::Struct(s) => {
                TypedDecl::Struct(s)
            }
            TypedDecl::Class => {
                TypedDecl::Class
            }
            TypedDecl::Enum => {
                TypedDecl::Enum
            }
            TypedDecl::Protocol => {
                TypedDecl::Protocol
            }
            TypedDecl::Extension => {
                TypedDecl::Extension
            }
        })
    }

    pub fn stmt(&self, s: TypedStmt) -> ResolverResult<TypedStmt> {
        ResolverResult::Ok(match s {
            TypedStmt::Expr(e) => {
                TypedStmt::Expr(e)
            }
            TypedStmt::Decl(d) => {
                TypedStmt::Decl(d)
            }
            TypedStmt::Assignment(a) => {
                TypedStmt::Assignment(a)
            }
            TypedStmt::Loop(l) => {
                TypedStmt::Loop(l)
            }
        })
    }
}
