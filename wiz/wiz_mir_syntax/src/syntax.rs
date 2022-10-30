use crate::span::Span;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct File {
    pub attrs: Vec<()>,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub id: usize,
    pub attrs: Vec<()>,
    pub visibility: (),
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Struct(VariantData),
    Union(VariantData),
    Function(FunctionDef),
    Const(),
    Static(),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VariantData {
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Field {
    pub id: usize,
    pub attrs: Vec<()>,
    pub visibility: (),
    pub span: Span,
    pub identifier: String,
    pub ty: (),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionDef {
    pub span: Span,
    pub body: Option<Block>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StatementKind {
    Expression,
    WhileLoop,
    Return,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
}
