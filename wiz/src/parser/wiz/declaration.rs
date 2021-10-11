use crate::parser::wiz::annotation::annotations;
use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::expression::expr;
use crate::parser::wiz::keywords::{
    as_keyword, fun_keyword, init_keyword, self_keyword, struct_keyword, use_keyword, val_keyword,
    var_keyword, where_keyword,
};
use crate::parser::wiz::lexical_structure::{
    identifier, trivia_piece_line_ending, whitespace0, whitespace1, whitespace_without_eol0,
};
use crate::parser::wiz::statement::stmts;
use crate::parser::wiz::type_::{type_, type_parameters};
use crate::syntax::annotation::Annotatable;
use crate::syntax::block::Block;
use crate::syntax::decl::{
    Decl, FunSyntax, InitializerSyntax, MethodSyntax, PackageName, StoredPropertySyntax,
    StructPropertySyntax, StructSyntax, UseSyntax, VarSyntax,
};
use crate::syntax::expr::Expr;
use crate::syntax::fun::arg_def::{ArgDef, SelfArgDefSyntax, ValueArgDef};
use crate::syntax::fun::body_def::FunBody;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{TypeName, TypeParam};
use crate::syntax::Syntax;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{
    AsChar, Compare, ExtendInto, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use std::ops::{Range, RangeFrom};

pub fn decl<I>(s: I) -> IResult<I, Decl>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            opt(tuple((annotations, whitespace0))),
            alt((use_decl, struct_decl, function_decl, var_decl)),
        )),
        |(a, d)| match a {
            Some((a, _)) => d.with_annotation(a),
            None => d,
        },
    )(s)
}

//region struct

pub fn struct_decl<I>(s: I) -> IResult<I, Decl>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(struct_syntax, |struct_syntax| Decl::Struct(struct_syntax))(s)
}

// <struct_decl> ::= "struct" <identifier> <type_parameters>? "{" <struct_properties> "}"
pub fn struct_syntax<I>(s: I) -> IResult<I, StructSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            struct_keyword,
            whitespace1,
            identifier,
            whitespace0,
            opt(type_parameters),
            opt(tuple((
                whitespace0,
                char('{'),
                whitespace0,
                struct_properties,
                whitespace0,
                char('}'),
            ))),
        )),
        |(_, _, name, _, params, body)| match body {
            Some((_, _, _, properties, _, _)) => StructSyntax {
                annotations: None,
                name,
                type_params: params,
                properties,
            },
            None => StructSyntax {
                annotations: None,
                name,
                type_params: params,
                properties: vec![],
            },
        },
    )(s)
}

// <struct_properties> ::= (<struct_property> ("\n" <struct_property>)* "\n"?)?
pub fn struct_properties<I>(s: I) -> IResult<I, Vec<StructPropertySyntax>>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        opt(tuple((
            struct_property,
            whitespace_without_eol0,
            many0(tuple((
                trivia_piece_line_ending,
                whitespace0,
                struct_property,
            ))),
            whitespace0,
        ))),
        |o| match o {
            None => vec![],
            Some((p, _, ps, _)) => {
                let mut ps: Vec<StructPropertySyntax> = ps.into_iter().map(|(_, _, p)| p).collect();
                ps.insert(0, p);
                ps
            }
        },
    )(s)
}

// <struct_property> ::= <stored_property>
//                     | <initializer>
pub fn struct_property<I>(s: I) -> IResult<I, StructPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    alt((stored_property, initializer, member_function))(s)
}

// <stored_property> ::= <mutable_stored_property> | <immutable_stored_property>
pub fn stored_property<I>(s: I) -> IResult<I, StructPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        alt((mutable_stored_property, immutable_stored_property)),
        |stored_property| StructPropertySyntax::StoredProperty(stored_property),
    )(s)
}

// <mutable_stored_property> ::= "var" <stored_property_body>
pub fn mutable_stored_property<I>(s: I) -> IResult<I, StoredPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((var_keyword, stored_property_body)),
        |(_, (name, _, typ))| StoredPropertySyntax {
            is_mut: true,
            name: name,
            type_: typ,
        },
    )(s)
}

// <immutable_stored_property> ::= "val" <stored_property_body>
pub fn immutable_stored_property<I>(s: I) -> IResult<I, StoredPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((val_keyword, stored_property_body)),
        |(_, (name, _, typ))| StoredPropertySyntax {
            is_mut: false,
            name: name,
            type_: typ,
        },
    )(s)
}

// <stored_property_body> ::= <identifier> ":" <type>
pub fn stored_property_body<I>(s: I) -> IResult<I, (String, char, TypeName)>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            whitespace1,
            identifier,
            whitespace0,
            char(':'),
            whitespace0,
            type_,
        )),
        |(_, name, _, c, _, typ)| (name, c, typ),
    )(s)
}

// <initializer> =:: "init" <function_value_parameters> <function_body>
pub fn initializer<I>(s: I) -> IResult<I, StructPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            init_keyword,
            whitespace0,
            function_value_parameters,
            whitespace0,
            function_body,
        )),
        |(_, _, args, _, body)| StructPropertySyntax::Init(InitializerSyntax { args, body }),
    )(s)
}

// <member_function> =:: <modifiers>? "fun" <identifire> <type_parameters>? <function_value_parameters> (":" <type>)? <type_constraints>? <function_body>?
pub fn member_function<I>(s: I) -> IResult<I, StructPropertySyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            fun_keyword,
            whitespace1,
            identifier,
            opt(type_parameters),
            function_value_parameters,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            opt(type_constraints),
            whitespace0,
            opt(function_body),
        )),
        |(f, _, name, type_params, args, _, return_type, _, t_constraints, _, body)| {
            StructPropertySyntax::Method(MethodSyntax {
                // modifiers: vec![],
                name: name,
                type_params,
                args,
                return_type: return_type.map(|(_, _, t)| t),
                body: body,
            })
        },
    )(s)
}

//endregion

//region func

pub fn function_decl<I>(s: I) -> IResult<I, Decl>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            fun_keyword,
            whitespace1,
            identifier,
            opt(type_parameters),
            function_value_parameters,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            opt(type_constraints),
            whitespace0,
            opt(function_body),
        )),
        |(f, _, name, type_params, args, _, return_type, _, t_constraints, _, body)| {
            Decl::Fun(FunSyntax {
                annotations: None,
                modifiers: vec![],
                name,
                type_params,
                arg_defs: args,
                return_type: return_type.map(|(_, _, t)| t),
                body,
            })
        },
    )(s)
}

pub fn function_value_parameters<I>(s: I) -> IResult<I, Vec<ArgDef>>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            char('('),
            opt(tuple((
                function_value_parameter,
                many0(map(tuple((comma, function_value_parameter)), |(_, a)| a)),
                opt(comma),
            ))),
            char(')'),
        )),
        |(_, args, _)| match args {
            Some((a, ar, _)) => vec![a].into_iter().chain(ar).collect(),
            None => Vec::new(),
        },
    )(s)
}

// <function_value_parameter> ::= (<function_value_label> <function_value_name> ":" <type> ("=" <expr>)?) | "self"
pub fn function_value_parameter<I>(s: I) -> IResult<I, ArgDef>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((
        map(
            tuple((
                whitespace0,
                opt(tuple((function_value_label, whitespace1))),
                function_value_name,
                whitespace0,
                char(':'),
                whitespace0,
                type_,
            )),
            |(_, label, name, _, _, _, typ)| {
                ArgDef::Value(ValueArgDef {
                    label: match label {
                        None => name.clone(),
                        Some((label, _)) => label,
                    },
                    name: name,
                    type_name: typ,
                })
            },
        ),
        map(
            tuple((opt(ampersand), whitespace0, self_keyword)),
            |(amp, ws, s): (_, _, I)| {
                ArgDef::Self_(SelfArgDefSyntax {
                    reference: amp.map(|a| TokenSyntax::new(a.to_string())),
                    self_: TokenSyntax::new(s.to_string()).with_leading_trivia(ws),
                })
            },
        ),
    ))(s)
}

pub fn function_value_label<I>(s: I) -> IResult<I, String>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    identifier(s)
}

pub fn function_value_name<I>(s: I) -> IResult<I, String>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    identifier(s)
}

pub fn type_constraints<I>(s: I) -> IResult<I, Vec<TypeParam>>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            where_keyword,
            whitespace1,
            type_constraint,
            whitespace0,
            opt(tuple((comma, whitespace0, type_constraint))),
            opt(comma),
        )),
        |(_, _, t, _, ts, _)| match ts {
            Some((_, _, ts)) => {
                vec![t, ts]
            }
            None => {
                vec![t]
            }
        },
    )(s)
}

pub fn type_constraint<I>(s: I) -> IResult<I, TypeParam>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((identifier, whitespace0, char(':'), whitespace0, type_)),
        |(id, _, _, _, typ)| TypeParam {
            name: id,
            type_constraints: Some(typ),
        },
    )(s)
}

pub fn function_body<I>(s: I) -> IResult<I, FunBody>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    alt((
        map(block, |b| FunBody::Block { block: b }),
        map(tuple((char('='), whitespace0, expr)), |(_, _, ex)| {
            FunBody::Expr { expr: ex }
        }),
    ))(s)
}

pub fn block<I>(s: I) -> IResult<I, Block>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((char('{'), whitespace0, stmts, whitespace0, char('}'))),
        |(_, _, stmts, _, _)| Block { body: stmts },
    )(s)
}

//endregion

//region var

pub fn var_decl<I>(s: I) -> IResult<I, Decl>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(var_syntax, |v| Decl::Var(v))(s)
}

pub fn var_syntax<I>(s: I) -> IResult<I, VarSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    alt((var, val))(s)
}

pub fn var<I>(s: I) -> IResult<I, VarSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((var_keyword, whitespace1, var_body)),
        |(_, _, (name, t, e))| VarSyntax {
            annotations: None,
            is_mut: true,
            name,
            type_: t,
            value: e,
        },
    )(s)
}

pub fn val<I>(s: I) -> IResult<I, VarSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((val_keyword, whitespace1, var_body)),
        |(_, _, (name, t, e))| VarSyntax {
            annotations: None,
            is_mut: false,
            name,
            type_: t,
            value: e,
        },
    )(s)
}

pub fn var_body<I>(s: I) -> IResult<I, (String, Option<TypeName>, Expr)>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            identifier,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            char('='),
            whitespace0,
            expr,
        )),
        |(name, _, t, _, _, _, e)| (name, t.map(|(_, _, t)| t), e),
    )(s)
}

//endregion

//region use
pub fn use_decl<I>(s: I) -> IResult<I, Decl>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(use_syntax, |u| Decl::Use(u))(s)
}

// <use> ::= "use" <package_name> ("as" <identifier>)?
pub fn use_syntax<I>(s: I) -> IResult<I, UseSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            use_keyword,
            whitespace1,
            package_name,
            opt(tuple((whitespace1, as_keyword, whitespace1, identifier))),
        )),
        |(_, _, pkg, alias)| UseSyntax {
            annotations: None,
            package_name: pkg,
            alias: alias.map(|(_, _, _, a)| a),
        },
    )(s)
}

// <package_name> ::= <identifier> ("::" <identifier>)*
pub fn package_name<I>(s: I) -> IResult<I, PackageName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((identifier, many0(tuple((tag("::"), identifier))))),
        |(i, is): (String, Vec<(I, String)>)| PackageName {
            names: vec![i]
                .into_iter()
                .chain(is.into_iter().map(|(_, i)| i))
                .collect(),
        },
    )(s)
}

//endregion

#[cfg(test)]
mod tests {
    use crate::parser::wiz::declaration::{
        block, function_body, function_decl, member_function, package_name, stored_property,
        struct_properties, struct_syntax, type_constraint, type_constraints, use_syntax, var_decl,
    };
    use crate::syntax::block::Block;
    use crate::syntax::decl::{
        Decl, FunSyntax, MethodSyntax, PackageName, StoredPropertySyntax, StructPropertySyntax,
        StructSyntax, UseSyntax, VarSyntax,
    };
    use crate::syntax::expr::{BinaryOperationSyntax, Expr, NameExprSyntax};
    use crate::syntax::fun::arg_def::{ArgDef, ValueArgDef};
    use crate::syntax::fun::body_def::FunBody;
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::stmt::Stmt;
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::type_name::{SimpleTypeName, TypeName, TypeParam};

    #[test]
    fn test_struct_properties() {
        assert_eq!(
            struct_properties(
                r"val a: Int64
                 val b: Int64
            "
            ),
            Ok((
                "",
                vec![
                    StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        is_mut: false,
                        name: "a".to_string(),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: "Int64".to_string(),
                            type_args: None
                        })
                    }),
                    StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        is_mut: false,
                        name: "b".to_string(),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: "Int64".to_string(),
                            type_args: None
                        })
                    }),
                ]
            ))
        )
    }

    #[test]
    fn test_stored_property() {
        assert_eq!(
            stored_property("val a: Int64"),
            Ok((
                "",
                StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                    is_mut: false,
                    name: "a".to_string(),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: "Int64".to_string(),
                        type_args: None
                    })
                })
            ))
        );
        assert_eq!(
            stored_property("var a: Int64"),
            Ok((
                "",
                StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                    is_mut: true,
                    name: "a".to_string(),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: "Int64".to_string(),
                        type_args: None
                    })
                })
            ))
        );
    }

    #[test]
    fn test_struct_syntax() {
        assert_eq!(
            struct_syntax(
                r##"struct A {
        var a: String
        }"##
            ),
            Ok((
                "",
                StructSyntax {
                    annotations: None,
                    name: "A".to_string(),
                    type_params: None,
                    properties: vec![StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        is_mut: true,
                        name: "a".to_string(),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: "String".to_string(),
                            type_args: None
                        })
                    })]
                }
            ))
        )
    }

    #[test]
    fn test_member_function() {
        assert_eq!(
            member_function("fun function() {}"),
            Ok((
                "",
                StructPropertySyntax::Method(MethodSyntax {
                    // modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    args: vec![],
                    return_type: None,
                    body: Some(FunBody::Block {
                        block: Block { body: vec![] }
                    }),
                })
            ))
        )
    }

    #[test]
    fn test_empty_block() {
        assert_eq!(block("{}"), Ok(("", Block { body: vec![] })))
    }

    #[test]
    fn test_block_with_int_literal() {
        assert_eq!(
            block("{1}"),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(
                        TokenSyntax::new("1".to_string())
                    )))]
                }
            ))
        )
    }

    #[test]
    fn test_block_with_binop_literal() {
        assert_eq!(
            block("{1+1}"),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "1".to_string()
                        )))),
                        kind: TokenSyntax::new("+".to_string()),
                        right: Box::new(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "1".to_string()
                        )))),
                    }))]
                }
            ))
        )
    }

    #[test]
    fn test_block() {
        assert_eq!(
            block(
                r"{
    1
}"
            ),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(
                        TokenSyntax::new("1".to_string())
                    )))]
                }
            ))
        )
    }

    #[test]
    fn test_function_body_block_case() {
        assert_eq!(
            function_body("{}"),
            Ok((
                "",
                FunBody::Block {
                    block: Block { body: vec![] }
                }
            ))
        )
    }

    #[test]
    fn test_function_body_expr_case() {
        assert_eq!(
            function_body("= name"),
            Ok((
                "",
                FunBody::Expr {
                    expr: Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "name".to_string()
                    })
                }
            ))
        )
    }

    #[test]
    fn test_function_decl() {
        assert_eq!(
            function_decl("fun function() {}"),
            Ok((
                "",
                Decl::Fun(FunSyntax {
                    annotations: None,
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    return_type: None,
                    body: Some(FunBody::Block {
                        block: Block { body: vec![] }
                    }),
                })
            ))
        )
    }

    #[test]
    fn test_function_no_body() {
        assert_eq!(
            function_decl("fun puts(_ item: String): Unit"),
            Ok((
                "",
                Decl::Fun(FunSyntax {
                    annotations: None,
                    modifiers: vec![],
                    name: "puts".to_string(),
                    type_params: None,
                    arg_defs: vec![ArgDef::Value(ValueArgDef {
                        label: "_".to_string(),
                        name: "item".to_string(),
                        type_name: TypeName::Simple(SimpleTypeName {
                            name: "String".to_string(),
                            type_args: None
                        })
                    })],
                    return_type: Some(TypeName::Simple(SimpleTypeName {
                        name: "Unit".to_string(),
                        type_args: None
                    })),
                    body: None,
                })
            ))
        )
    }

    #[test]
    fn test_function_short_label() {
        assert_eq!(
            function_decl("fun puts(item: String): Unit"),
            Ok((
                "",
                Decl::Fun(FunSyntax {
                    annotations: None,
                    modifiers: vec![],
                    name: "puts".to_string(),
                    type_params: None,
                    arg_defs: vec![ArgDef::Value(ValueArgDef {
                        label: "item".to_string(),
                        name: "item".to_string(),
                        type_name: TypeName::Simple(SimpleTypeName {
                            name: "String".to_string(),
                            type_args: None
                        })
                    })],
                    return_type: Some(TypeName::Simple(SimpleTypeName {
                        name: "Unit".to_string(),
                        type_args: None
                    })),
                    body: None,
                })
            ))
        )
    }

    #[test]
    fn test_var_decl() {
        assert_eq!(
            var_decl("val a: Int = 1"),
            Ok((
                "",
                Decl::Var(VarSyntax {
                    annotations: None,
                    is_mut: false,
                    name: "a".to_string(),
                    type_: Some(TypeName::Simple(SimpleTypeName {
                        name: "Int".to_string(),
                        type_args: None
                    })),
                    value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new("1".to_string())))
                })
            ))
        )
    }

    #[test]
    fn test_var_decl_without_type() {
        assert_eq!(
            var_decl("val a = 1"),
            Ok((
                "",
                Decl::Var(VarSyntax {
                    annotations: None,
                    is_mut: false,
                    name: "a".to_string(),
                    type_: None,
                    value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new("1".to_string())))
                })
            ))
        )
    }

    #[test]
    fn test_type_constraint() {
        assert_eq!(
            type_constraint("T: Printable"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Printable".to_string(),
                        type_args: None
                    }))
                }
            ))
        )
    }

    #[test]
    fn test_type_constraints() {
        assert_eq!(
            type_constraints("where T: Printable,"),
            Ok((
                "",
                vec![TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Printable".to_string(),
                        type_args: None
                    }))
                },]
            ))
        );
        assert_eq!(
            type_constraints("where T: Printable, T: DebugPrintable"),
            Ok((
                "",
                vec![
                    TypeParam {
                        name: "T".to_string(),
                        type_constraints: Some(TypeName::Simple(SimpleTypeName {
                            name: "Printable".to_string(),
                            type_args: None
                        }))
                    },
                    TypeParam {
                        name: "T".to_string(),
                        type_constraints: Some(TypeName::Simple(SimpleTypeName {
                            name: "DebugPrintable".to_string(),
                            type_args: None
                        }))
                    }
                ]
            ))
        );
        assert_eq!(
            type_constraints("where T: Printable, T: DebugPrintable,"),
            Ok((
                "",
                vec![
                    TypeParam {
                        name: "T".to_string(),
                        type_constraints: Some(TypeName::Simple(SimpleTypeName {
                            name: "Printable".to_string(),
                            type_args: None
                        }))
                    },
                    TypeParam {
                        name: "T".to_string(),
                        type_constraints: Some(TypeName::Simple(SimpleTypeName {
                            name: "DebugPrintable".to_string(),
                            type_args: None
                        }))
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_package_name() {
        assert_eq!(
            package_name("abc"),
            Ok((
                "",
                PackageName {
                    names: vec![String::from("abc")]
                }
            ))
        );
        assert_eq!(
            package_name("abc::def"),
            Ok((
                "",
                PackageName {
                    names: vec![String::from("abc"), String::from("def")]
                }
            ))
        );
    }

    #[test]
    fn test_use() {
        assert_eq!(
            use_syntax("use abc"),
            Ok((
                "",
                UseSyntax {
                    annotations: None,
                    package_name: PackageName {
                        names: vec![String::from("abc")]
                    },
                    alias: None
                }
            ))
        );
        assert_eq!(
            use_syntax("use abc as def"),
            Ok((
                "",
                UseSyntax {
                    annotations: None,
                    package_name: PackageName {
                        names: vec![String::from("abc")]
                    },
                    alias: Some(String::from("def"))
                }
            ))
        );
    }
}
