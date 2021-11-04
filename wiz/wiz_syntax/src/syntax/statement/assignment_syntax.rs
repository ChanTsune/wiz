use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AssignmentStmt {
    Assignment(AssignmentSyntax),
    AssignmentAndOperator(AssignmentAndOperatorSyntax),
}

impl Syntax for AssignmentStmt {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            AssignmentStmt::Assignment(a) => {
                AssignmentStmt::Assignment(a.with_leading_trivia(trivia))
            }
            AssignmentStmt::AssignmentAndOperator(a) => {
                AssignmentStmt::AssignmentAndOperator(a.with_leading_trivia(trivia))
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            AssignmentStmt::Assignment(a) => {
                AssignmentStmt::Assignment(a.with_trailing_trivia(trivia))
            }
            AssignmentStmt::AssignmentAndOperator(a) => {
                AssignmentStmt::AssignmentAndOperator(a.with_trailing_trivia(trivia))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub target: Expr,
    pub operator: TokenSyntax,
    pub value: Expr,
}

impl Syntax for AssignmentSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target.with_leading_trivia(trivia),
            operator: self.operator,
            value: self.value
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            operator: self.operator,
            value: self.value.with_trailing_trivia(trivia)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentAndOperatorSyntax {
    pub target: Expr,
    pub operator: TokenSyntax,
    pub value: Expr,
}

impl Syntax for AssignmentAndOperatorSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target.with_leading_trivia(trivia),
            operator: self.operator,
            value: self.value
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            operator: self.operator,
            value: self.value.with_trailing_trivia(trivia)
        }
    }
}
