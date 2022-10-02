use crate::typed_decl::TypedDecl;
use crate::typed_use::TypedUse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedSpellBook {
    pub name: String,
    pub uses: Vec<TypedUse>,
    pub body: Vec<TypedDecl>,
}
