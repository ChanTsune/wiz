use nom::{IResult, Parser};
use crate::ast::decl::Decl;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use crate::parser::nom::keywords::{fun_keyword, where_keyword};
use crate::parser::nom::lexical_structure::{identifier, whitespace1, whitespace0};
use nom::character::complete::char;
use crate::parser::nom::type_::type_;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::block::Block;
use crate::parser::nom::expression::expr;
use crate::ast::type_name::{TypeName, TypeParam};
use nom::multi::many0;
use crate::parser::nom::stmts;

pub fn decl(s: &str) -> IResult<&str, Decl> {
    alt((
        function_decl,
        function_decl,
    ))(s)
}

pub fn function_decl(s: &str) -> IResult<&str, Decl> {
    map(tuple((
        fun_keyword,
        whitespace1,
        identifier,
        // opt(type_parameters),
        function_value_parameters,
        whitespace0,
        opt(tuple((
            char(':'),
            type_,
        ))),
        whitespace0,
        opt(type_constraints),
        whitespace0,
        opt(function_body),
    )), |(
             f,
             _,
             name,
             /* type_params, */
             args,
             _,
             return_type,
             _,
             t_constraints,
             _,
             body
         )| {
        Decl::Fun {
            modifiers: vec![],
            name: name,
            arg_defs: args,
            return_type: match return_type {
                Some((_, type_)) => { type_ }
                None => { TypeName { name: "Unit".parse().unwrap(), type_params: vec![] } }
            },
            body: body,
        }
    })(s)
}

pub fn function_value_parameters(s: &str) -> IResult<&str, Vec<ArgDef>> {
    map(tuple((
        char('('),
        opt(tuple((
            function_value_parameter,
            many0(map(tuple((
                char(','),
                function_value_parameter,
            )), |(_, a)| { a })),
            opt(char(',')),
        ))),
        char(')'),
    )), |(_, args, _)| {
        match args {
            Some((a, mut ar, _)) => {
                let mut t = vec![a];
                t.append(&mut ar);
                t
            }
            None => { Vec::new() }
        }
    })(s)
}

pub fn function_value_parameter(s: &str) -> IResult<&str, ArgDef> {
    map(tuple((
        whitespace0,
        function_value_label,
        whitespace1,
        function_value_name,
        whitespace0,
        char(':'),
        whitespace0,
        type_,
    )), |(_, label, _, name, _, _, _, typ)| {
        ArgDef {
            label: label,
            name: name,
            type_name: typ,
        }
    })(s)
}

pub fn function_value_label(s: &str) -> IResult<&str, String> {
    identifier(s)
}

pub fn function_value_name(s: &str) -> IResult<&str, String> {
    identifier(s)
}

pub fn type_constraints(s: &str) -> IResult<&str, Vec<TypeParam>> {
    map(tuple((
        where_keyword,
        type_constraint,
        opt(tuple((
            char(','),
            type_constraint,
        )))
    )), |(_, t, ts)| {
        match ts {
            Some((_, ts)) => {
                vec![t, ts]
            }
            None => { vec![t] }
        }
    })(s)
}

pub fn type_constraint(s: &str) -> IResult<&str, TypeParam> {
    map(tuple((
        identifier,
        char(':'),
        type_,
    )), |(id, _, typ)| {
        TypeParam { name: id, type_constraints: vec![typ] }
    })(s)
}

pub fn function_body(s: &str) -> IResult<&str, FunBody> {
    alt((
        map(block, |b| { FunBody::Block { block: b } }),
        map(tuple((
            char('='),
            expr,
        )), |(_, ex)| { FunBody::Expr { expr: ex } },
        )))(s)
}

pub fn block(s: &str) -> IResult<&str, Block> {
    map(tuple((
        char('{'),
        whitespace0,
        stmts,
        whitespace0,
        char('}'),
    )), |(_, _, stmts, _, _)| {
        Block { body: stmts }
    })(s)
}

#[cfg(test)]
mod test {
    use crate::parser::nom::declaration::{block, function_decl, function_body};
    use crate::ast::block::Block;
    use crate::ast::stmt::Stmt;
    use crate::ast::literal::Literal;
    use crate::ast::expr::Expr;
    use crate::ast::fun::body_def::FunBody;
    use crate::ast::decl::Decl;
    use crate::ast::type_name::TypeName;

    #[test]
    fn test_empty_block() {
        assert_eq!(block("{}"), Ok(("", Block { body: vec![] })))
    }

    #[test]
    fn test_block_with_int_literal() {
        assert_eq!(block("{1}"), Ok(("", Block { body: vec![Stmt::Expr { expr: Expr::Literal { literal: Literal::IntegerLiteral { value: "1".to_string() } } }] })))
    }

    #[test]
    fn test_block_with_binop_literal() {
        assert_eq!(block("{1+1}"), Ok(("", Block {
            body: vec![
                Stmt::Expr {
                    expr: Expr::BinOp {
                        left: Box::new(Expr::Literal { literal: Literal::IntegerLiteral { value: "1".to_string() } }),
                        kind: "+".to_string(),
                        right: Box::new(Expr::Literal { literal: Literal::IntegerLiteral { value: "1".to_string() } }),
                    }
                }
            ]
        })))
    }

    #[test]
    fn test_block() {
        assert_eq!(block(
            r"{
    1
}"), Ok(("", Block { body: vec![Stmt::Expr { expr: Expr::Literal { literal: Literal::IntegerLiteral { value: "1".to_string() } } }] })))
    }

    #[test]
    fn test_function_body_block_case() {
        assert_eq!(function_body("{}"), Ok(("", FunBody::Block { block: Block { body: vec![] } })))
    }

    #[test]
    fn test_function_body_expr_case() {
        assert_eq!(function_body("=name"), Ok(("", FunBody::Expr { expr: Expr::Name { name: "name".to_string() } })))
    }

    #[test]
    fn test_function_decl() {
        assert_eq!(function_decl("fun function() {}"), Ok(("", Decl::Fun {
            modifiers: vec![],
            name: "function".to_string(),
            arg_defs: vec![],
            return_type: TypeName { name: "Unit".to_string(), type_params: vec![] },
            body: Some(FunBody::Block { block: Block { body: vec![] } }),
        })))
    }
}
