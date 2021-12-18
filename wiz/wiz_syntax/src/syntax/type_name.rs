use crate::syntax::list::{ElementSyntax, ListSyntax};
use crate::syntax::name_space::NameSpaceSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    NameSpaced(Box<UserTypeName>),
    Simple(SimpleTypeName),
    Decorated(Box<DecoratedTypeName>),
}

impl Syntax for TypeName {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            TypeName::NameSpaced(n) => {
                TypeName::NameSpaced(Box::new(n.with_leading_trivia(trivia)))
            }
            TypeName::Simple(s) => TypeName::Simple(s.with_leading_trivia(trivia)),
            TypeName::Decorated(d) => TypeName::Decorated(Box::new(d.with_leading_trivia(trivia))),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            TypeName::NameSpaced(n) => {
                TypeName::NameSpaced(Box::new(n.with_trailing_trivia(trivia)))
            }
            TypeName::Simple(s) => TypeName::Simple(s.with_trailing_trivia(trivia)),
            TypeName::Decorated(d) => TypeName::Decorated(Box::new(d.with_trailing_trivia(trivia))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserTypeName {
    pub name_space: NameSpaceSyntax,
    pub type_name: TypeName,
}

impl Syntax for UserTypeName {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            name_space: self.name_space.with_leading_trivia(trivia),
            type_name: self.type_name,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            name_space: self.name_space,
            type_name: self.type_name.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SimpleTypeName {
    pub name: TokenSyntax,
    pub type_args: Option<TypeArgumentListSyntax>,
}

impl Syntax for SimpleTypeName {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            name: self.name.with_leading_trivia(trivia),
            type_args: None,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.type_args {
            None => Self {
                name: self.name.with_trailing_trivia(trivia),
                type_args: None,
            },
            Some(type_args) => Self {
                name: self.name,
                type_args: Some(type_args.with_trailing_trivia(trivia)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecoratedTypeName {
    pub decoration: TokenSyntax,
    pub type_: TypeName,
}

impl Syntax for DecoratedTypeName {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            decoration: self.decoration.with_leading_trivia(trivia),
            type_: self.type_,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            decoration: self.decoration,
            type_: self.type_.with_trailing_trivia(trivia),
        }
    }
}

pub type TypeParameterListSyntax = ListSyntax<TypeParam>;
pub type TypeParameterElementSyntax = ElementSyntax<TypeParam>;

impl TypeParameterListSyntax {
    fn new() -> Self {
        Self {
            open: TokenSyntax::from("<"),
            elements: vec![],
            close: TokenSyntax::from(">"),
        }
    }
}

impl Default for TypeParameterListSyntax {
    fn default() -> Self {
        Self::new()
    }
}

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
            Some(type_constraint) => Self {
                name: self.name,
                type_constraint: Some(type_constraint.with_trailing_trivia(trivia)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeConstraintSyntax {
    pub sep: TokenSyntax,
    pub constraint: TypeName,
}

impl Syntax for TypeConstraintSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            sep: self.sep.with_leading_trivia(trivia),
            constraint: self.constraint,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            sep: self.sep,
            constraint: self.constraint.with_trailing_trivia(trivia),
        }
    }
}

pub type TypeArgumentListSyntax = ListSyntax<TypeName>;
pub type TypeArgumentElementSyntax = ElementSyntax<TypeName>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeConstraintsSyntax {
    pub where_keyword: TokenSyntax,
    pub type_constraints: Vec<TypeConstraintElementSyntax>,
}

impl Syntax for TypeConstraintsSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            where_keyword: self.where_keyword.with_leading_trivia(trivia),
            type_constraints: self.type_constraints,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        if self.type_constraints.is_empty() {
            Self {
                where_keyword: self.where_keyword.with_trailing_trivia(trivia),
                type_constraints: self.type_constraints,
            }
        } else {
            let mut type_constraints = self.type_constraints.clone();
            let t = type_constraints.pop().unwrap().with_trailing_trivia(trivia);
            type_constraints.push(t);
            Self {
                where_keyword: self.where_keyword,
                type_constraints,
            }
        }
    }
}

pub type TypeConstraintElementSyntax = ElementSyntax<TypeParam>;
