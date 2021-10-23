use crate::syntax::name_space::NameSpaceSyntax;
use crate::syntax::node::SyntaxNode;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    NameSpaced(Box<NameSpacedTypeName>),
    Simple(SimpleTypeName),
    Decorated(Box<DecoratedTypeName>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameSpacedTypeName {
    pub name_space: NameSpaceSyntax,
    pub type_name: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SimpleTypeName {
    pub name: TokenSyntax,
    pub type_args: Option<Vec<TypeName>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecoratedTypeName {
    pub decoration: TokenSyntax,
    pub type_: TypeName,
}

impl SyntaxNode for TypeName {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeParameterListSyntax {
    pub open: TokenSyntax,
    pub elements: Vec<TypeParameterElementSyntax>,
    pub close: TokenSyntax,
}

impl TypeParameterListSyntax {
    fn new() -> Self {
        Self {
            open: TokenSyntax::from("<"),
            elements: vec![],
            close: TokenSyntax::from(">"),
        }
    }
}

impl SyntaxNode for TypeParameterListSyntax {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeParameterElementSyntax {
    pub element: TypeParam,
    pub trailing_comma: Option<TokenSyntax>,
}

impl SyntaxNode for TypeParameterElementSyntax {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeParam {
    pub name: TokenSyntax,
    pub type_constraint: Option<TypeConstraintSyntax>,
}

impl Syntax for TypeParam {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            name: self.name.with_leading_trivia(trivia),
            type_constraint: self.type_constraint,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.type_constraint {
            None => Self {
                name: self.name.with_trailing_trivia(trivia),
                type_constraint: None,
            },
            Some(type_constraint) => {
                todo!(
                    r"
                Self {{
                    name: self.name,
                    type_constraint: Some(type_constraint.with_trailing_trivia(trivia))
                }}
                "
                )
            }
        }
    }
}

impl SyntaxNode for TypeParam {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeConstraintSyntax {
    pub sep: TokenSyntax,
    pub constraint: TypeName,
}

impl SyntaxNode for TypeConstraintSyntax {}
