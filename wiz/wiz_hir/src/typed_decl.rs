use wiz_data_structure::annotation::TypedAnnotations;
use crate::typed_expr::{TypedExpr, TypedExprKind};
use crate::typed_file::TypedSpellBook;
use crate::typed_stmt::TypedBlock;
use crate::typed_type::{Package, TypedArgType, TypedFunctionType, TypedType, TypedTypeParam};
use crate::typed_type_constraint::TypedTypeConstraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedTopLevelDecl {
    pub annotations: TypedAnnotations,
    pub package: Package,
    pub modifiers: Vec<String>,
    pub kind: TypedDeclKind,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedDeclKind {
    Var(TypedVar),
    Fun(TypedFun),
    Struct(TypedStruct),
    Module(TypedModule),
    Enum,
    Protocol(TypedProtocol),
    Extension(TypedExtension),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedVar {
    pub is_mut: bool,
    pub name: String,
    pub type_: Option<TypedType>,
    pub value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedFun {
    pub name: String,
    pub type_params: Option<Vec<TypedTypeParam>>,
    pub type_constraints: Option<Vec<TypedTypeConstraint>>,
    pub arg_defs: Vec<TypedArgDef>,
    pub body: Option<TypedFunBody>,
    pub return_type: TypedType,
}

impl TypedFun {
    pub fn size(ty: TypedType) -> Self {
        TypedFun {
            name: "size".to_string(),
            type_params: None,
            type_constraints: None,
            arg_defs: vec![],
            body: Some(TypedFunBody::Expr(TypedExpr::new(
                TypedExprKind::SizeOf(ty),
                Some(TypedType::usize()),
            ))),
            return_type: TypedType::usize(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedArgDef {
    pub label: String,
    pub name: String,
    pub type_: TypedType,
}

impl TypedArgDef {
    pub fn to_arg_type(&self) -> TypedArgType {
        TypedArgType {
            label: self.label.clone(),
            typ: self.type_.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedFunBody {
    Expr(TypedExpr),
    Block(TypedBlock),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStruct {
    pub name: String,
    pub type_params: Option<Vec<TypedTypeParam>>,
    pub stored_properties: Vec<TypedStoredProperty>,
    pub computed_properties: Vec<TypedComputedProperty>,
    pub member_functions: Vec<TypedFun>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStoredProperty {
    pub name: String,
    pub type_: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedComputedProperty {
    pub name: String,
    pub type_: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedExtension {
    pub name: TypedType,
    pub protocol: Option<TypedType>,
    pub computed_properties: Vec<TypedComputedProperty>,
    pub member_functions: Vec<TypedFun>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedProtocol {
    pub name: String,
    pub type_params: Option<Vec<TypedTypeParam>>,
    pub computed_properties: Vec<TypedComputedProperty>,
    pub member_functions: Vec<TypedFun>,
}

impl TypedFun {
    pub fn type_(&self) -> TypedType {
        TypedType::Function(Box::new(TypedFunctionType {
            arguments: self.arg_defs.iter().map(|a| a.to_arg_type()).collect(),
            return_type: self.return_type.clone(),
        }))
    }

    pub fn is_generic(&self) -> bool {
        self.type_params.is_some()
    }
}

pub type TypedModule = TypedSpellBook;
