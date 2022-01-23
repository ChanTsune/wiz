use crate::parser::wiz::annotation::annotations_syntax;
use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::expression::expr;
use crate::parser::wiz::keywords::{
    as_keyword, deinit_keyword, extension_keyword, fun_keyword, init_keyword, protocol_keyword,
    self_keyword, struct_keyword, use_keyword, val_keyword, var_keyword, where_keyword,
};
use crate::parser::wiz::lexical_structure::{
    identifier, trivia_piece_line_ending, whitespace0, whitespace1, whitespace_without_eol0,
};
use crate::parser::wiz::statement::stmts;
use crate::parser::wiz::type_::{type_, type_parameters};
use crate::syntax::annotation::Annotatable;
use crate::syntax::block::BlockSyntax;
use crate::syntax::declaration::fun_syntax::{
    ArgDef, ArgDefElementSyntax, ArgDefListSyntax, FunBody, FunSyntax, SelfArgDefSyntax,
    ValueArgDef,
};
use crate::syntax::declaration::{
    AliasSyntax, Decl, DeinitializerSyntax, ExtensionSyntax, InitializerSyntax, PackageName,
    ProtocolConformSyntax, StoredPropertySyntax, StructPropertySyntax, StructSyntax, UseSyntax,
};
use crate::syntax::declaration::{PackageNameElement, VarSyntax};
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{
    TypeConstraintElementSyntax, TypeConstraintSyntax, TypeConstraintsSyntax, TypeName, TypeParam,
};
use crate::syntax::Syntax;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
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
            opt(tuple((annotations_syntax, whitespace0))),
            alt((
                use_decl,
                struct_decl,
                function_decl,
                var_decl,
                extension_decl,
            )),
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
    map(struct_syntax, Decl::Struct)(s)
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
            alt((struct_keyword, protocol_keyword)),
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
        |(struct_keyword, nws, name, _, params, body)| match body {
            Some((_, open, _, properties, _, close)) => StructSyntax {
                annotations: None,
                struct_keyword: TokenSyntax::from(struct_keyword),
                name: TokenSyntax::from(name).with_leading_trivia(nws),
                type_params: params,
                open: TokenSyntax::from(open),
                properties,
                close: TokenSyntax::from(close),
            },
            None => StructSyntax {
                annotations: None,
                struct_keyword: TokenSyntax::from(struct_keyword),
                name: TokenSyntax::from(name).with_leading_trivia(nws),
                type_params: params,
                open: Default::default(),
                properties: vec![],
                close: Default::default(),
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
//                     | <deinitializer>
//                     | <member_function>
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
    alt((stored_property, initializer, deinitializer, member_function))(s)
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
        StructPropertySyntax::StoredProperty,
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
        |(var, (name, _, typ))| StoredPropertySyntax {
            mutability_keyword: TokenSyntax::from(var),
            name: TokenSyntax::from(name),
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
        |(val, (name, _, typ))| StoredPropertySyntax {
            mutability_keyword: TokenSyntax::from(val),
            name: TokenSyntax::from(name),
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
        |(init, ws, args, _, body): (I, _, _, _, _)| {
            StructPropertySyntax::Init(InitializerSyntax {
                init_keyword: TokenSyntax::from(init).with_trailing_trivia(ws),
                args,
                body,
            })
        },
    )(s)
}

// <initializer> =:: "deinit" <function_body>
pub fn deinitializer<I>(s: I) -> IResult<I, StructPropertySyntax>
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
        tuple((deinit_keyword, whitespace0, function_body)),
        |(deinit, ws, body): (I, _, _)| {
            StructPropertySyntax::Deinit(DeinitializerSyntax {
                deinit_keyword: TokenSyntax::from(deinit).with_trailing_trivia(ws),
                body,
            })
        },
    )(s)
}

// <member_function> =:: <modifiers>? "fun" <identifier> <type_parameters>? <function_value_parameters> (":" <type>)? <type_constraints>? <function_body>?
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
    map(function_syntax, StructPropertySyntax::Method)(s)
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
    map(function_syntax, Decl::Fun)(s)
}

pub fn function_syntax<I>(s: I) -> IResult<I, FunSyntax>
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
        |(f, ws, name, type_params, args, _, return_type, _, type_constraints, _, body)| {
            FunSyntax {
                annotations: None,
                modifiers: Default::default(),
                fun_keyword: TokenSyntax::from(f),
                name: TokenSyntax::from(name).with_leading_trivia(ws),
                type_params,
                arg_defs: args,
                return_type: return_type.map(|(_, _, t)| t),
                type_constraints,
                body,
            }
        },
    )(s)
}

pub fn function_value_parameters<I>(s: I) -> IResult<I, ArgDefListSyntax>
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
            many0(tuple((
                whitespace0,
                function_value_parameter,
                whitespace0,
                comma,
            ))),
            whitespace0,
            opt(function_value_parameter),
            whitespace0,
            char(')'),
        )),
        |(open, elements, ws, element, tws, close)| {
            let mut close = TokenSyntax::from(close);
            let mut elements: Vec<_> = elements
                .into_iter()
                .map(|(lws, e, rws, c)| ArgDefElementSyntax {
                    element: e.with_leading_trivia(lws),
                    trailing_comma: Some(TokenSyntax::from(c).with_leading_trivia(rws)),
                })
                .collect();
            match element {
                None => {
                    close = close.with_leading_trivia(ws + tws);
                }
                Some(e) => {
                    elements.push(ArgDefElementSyntax {
                        element: e.with_leading_trivia(ws),
                        trailing_comma: None,
                    });
                    close = close.with_leading_trivia(tws);
                }
            };

            ArgDefListSyntax {
                open: TokenSyntax::from(open),
                elements,
                close,
            }
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
                    label: label
                        .map(|(label, ws)| TokenSyntax::from(label).with_trailing_trivia(ws)),
                    name: TokenSyntax::from(name),
                    type_name: typ,
                })
            },
        ),
        map(
            tuple((opt(ampersand), whitespace0, self_keyword)),
            |(amp, ws, s): (_, _, I)| {
                ArgDef::Self_(SelfArgDefSyntax {
                    reference: amp.map(TokenSyntax::from),
                    self_: TokenSyntax::from(s).with_leading_trivia(ws),
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

pub fn type_constraints<I>(s: I) -> IResult<I, TypeConstraintsSyntax>
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
            many0(tuple((whitespace0, type_constraint, whitespace0, comma))),
            whitespace0,
            opt(type_constraint),
        )),
        |(where_keyword, ws, t, lws, ts)| {
            let mut type_constraints: Vec<_> = t
                .into_iter()
                .map(|(lws, ta, rws, c)| TypeConstraintElementSyntax {
                    element: ta.with_leading_trivia(lws),
                    trailing_comma: Some(TokenSyntax::from(c).with_leading_trivia(rws)),
                })
                .collect();
            if let Some(ts) = ts {
                type_constraints.push(TypeConstraintElementSyntax {
                    element: ts.with_leading_trivia(lws),
                    trailing_comma: None,
                })
            };
            if !type_constraints.is_empty() {
                let t = type_constraints.remove(0).with_leading_trivia(ws);
                type_constraints.insert(0, t);
            };
            TypeConstraintsSyntax {
                where_keyword: TokenSyntax::from(where_keyword),
                type_constraints,
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
        |(id, lws, sep, rws, typ)| TypeParam {
            name: TokenSyntax::from(id),
            type_constraint: Some(TypeConstraintSyntax {
                sep: TokenSyntax::from(sep)
                    .with_leading_trivia(lws)
                    .with_trailing_trivia(rws),
                constraint: typ,
            }),
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
        map(block, FunBody::Block),
        map(tuple((char('='), whitespace0, expr)), |(_, _, ex)| {
            FunBody::Expr(ex)
        }),
    ))(s)
}

pub fn block<I>(s: I) -> IResult<I, BlockSyntax>
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
        |(open, ows, stmts, cws, close)| BlockSyntax {
            open: TokenSyntax::from(open).with_trailing_trivia(ows),
            body: stmts,
            close: TokenSyntax::from(close).with_leading_trivia(cws),
        },
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
    map(var_syntax, Decl::Var)(s)
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
    map(
        tuple((alt((var_keyword, val_keyword)), whitespace1, var_body)),
        |(mutability_keyword, ws, (name, t, e)): (I, _, _)| VarSyntax {
            annotations: None,
            mutability_keyword: TokenSyntax::from(mutability_keyword).with_trailing_trivia(ws),
            name: TokenSyntax::from(name),
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
    map(use_syntax, Decl::Use)(s)
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
            alt((identifier, map(tag("*"), |i: I| i.to_string()))),
            opt(tuple((whitespace1, as_keyword, whitespace1, identifier))),
        )),
        |(u, _, pkg, n, alias)| UseSyntax {
            annotations: None,
            use_keyword: TokenSyntax::from(u),
            package_name: pkg,
            used_name: TokenSyntax::from(n),
            alias: alias.map(|(lws, a, rws, n)| AliasSyntax {
                as_keyword: TokenSyntax::from(a).with_leading_trivia(lws),
                name: TokenSyntax::from(n).with_leading_trivia(rws),
            }),
        },
    )(s)
}

// <package_name> ::= (<identifier> "::")*
pub fn package_name<I>(s: I) -> IResult<I, PackageName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        many0(tuple((identifier, tag("::")))),
        |i: Vec<(String, I)>| PackageName {
            names: i
                .into_iter()
                .map(|(name, sep)| PackageNameElement {
                    name: TokenSyntax::from(name),
                    sep: TokenSyntax::from(sep),
                })
                .collect(),
        },
    )(s)
}

//endregion

//region extension
pub fn extension_decl<I>(s: I) -> IResult<I, Decl>
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
    map(extension_syntax, Decl::Extension)(s)
}

pub fn extension_syntax<I>(s: I) -> IResult<I, ExtensionSyntax>
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
            extension_keyword,
            opt(tuple((whitespace0, type_parameters))),
            whitespace1,
            type_,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            opt(type_constraints),
            whitespace0,
            char('{'),
            whitespace0,
            struct_properties,
            whitespace0,
            char('}'),
        )),
        |(kw, tp, ws, n, _, protocol, _, tc, ws1, _, ws2, properties, _, _)| ExtensionSyntax {
            annotations: None,
            modifiers: Default::default(),
            extension_keyword: TokenSyntax::from(kw),
            type_params: tp.map(|(t, tp)| tp.with_leading_trivia(t)),
            name: n,
            protocol_extension: protocol.map(|(colon, _, typ)| ProtocolConformSyntax {
                colon: TokenSyntax::from(colon),
                protocol: typ,
            }),
            type_constraints: tc,
            properties,
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
    use crate::syntax::block::BlockSyntax;
    use crate::syntax::declaration::fun_syntax::{
        ArgDef, ArgDefElementSyntax, ArgDefListSyntax, FunBody, FunSyntax, ValueArgDef,
    };
    use crate::syntax::declaration::{
        AliasSyntax, Decl, PackageName, StoredPropertySyntax, StructPropertySyntax, StructSyntax,
        UseSyntax,
    };
    use crate::syntax::declaration::{PackageNameElement, VarSyntax};
    use crate::syntax::expression::{BinaryOperationSyntax, Expr, NameExprSyntax};
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::modifier::ModifiersSyntax;
    use crate::syntax::statement::Stmt;
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::{Trivia, TriviaPiece};
    use crate::syntax::type_name::{
        SimpleTypeName, TypeConstraintElementSyntax, TypeConstraintSyntax, TypeConstraintsSyntax,
        TypeName, TypeParam,
    };
    use crate::syntax::Syntax;

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
                        mutability_keyword: TokenSyntax::from("val"),
                        name: TokenSyntax::from("a"),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int64"),
                            type_args: None
                        })
                    }),
                    StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        mutability_keyword: TokenSyntax::from("val"),
                        name: TokenSyntax::from("b"),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int64"),
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
                    mutability_keyword: TokenSyntax::from("val"),
                    name: TokenSyntax::from("a"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int64"),
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
                    mutability_keyword: TokenSyntax::from("var"),
                    name: TokenSyntax::from("a"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int64"),
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
                    struct_keyword: TokenSyntax::from("struct"),
                    name: TokenSyntax::from("A")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    type_params: None,
                    open: TokenSyntax::from("{"),
                    properties: vec![StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        mutability_keyword: TokenSyntax::from("var"),
                        name: TokenSyntax::from("a"),
                        type_: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("String"),
                            type_args: None
                        })
                    })],
                    close: TokenSyntax::from("}")
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
                StructPropertySyntax::Method(FunSyntax {
                    annotations: None,
                    modifiers: ModifiersSyntax::new(),
                    fun_keyword: TokenSyntax::from("fun"),
                    name: TokenSyntax::from("function")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    type_params: None,
                    arg_defs: ArgDefListSyntax::default(),
                    return_type: None,
                    type_constraints: None,
                    body: Some(FunBody::Block(BlockSyntax {
                        open: TokenSyntax::from("{"),
                        body: vec![],
                        close: TokenSyntax::from("}")
                    })),
                })
            ))
        )
    }

    #[test]
    fn test_empty_block() {
        assert_eq!(
            block("{}"),
            Ok((
                "",
                BlockSyntax {
                    open: TokenSyntax::from("{"),
                    body: vec![],
                    close: TokenSyntax::from("}")
                }
            ))
        )
    }

    #[test]
    fn test_block_with_int_literal() {
        assert_eq!(
            block("{1}"),
            Ok((
                "",
                BlockSyntax {
                    open: TokenSyntax::from("{"),
                    body: vec![Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(
                        TokenSyntax::from("1")
                    )))],
                    close: TokenSyntax::from("}")
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
                BlockSyntax {
                    open: TokenSyntax::from("{"),
                    body: vec![Stmt::Expr(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from(
                            "1"
                        )))),
                        operator: TokenSyntax::from("+"),
                        right: Box::new(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from(
                            "1"
                        )))),
                    }))],
                    close: TokenSyntax::from("}")
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
                BlockSyntax {
                    open: TokenSyntax::from("{").with_trailing_trivia(Trivia::from(vec![
                        TriviaPiece::Newlines(1),
                        TriviaPiece::Spaces(4)
                    ])),
                    body: vec![Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(
                        TokenSyntax::from("1")
                    )))],
                    close: TokenSyntax::from("}")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Newlines(1)))
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
                FunBody::Block(BlockSyntax {
                    open: TokenSyntax::from("{"),
                    body: vec![],
                    close: TokenSyntax::from("}")
                })
            ))
        )
    }

    #[test]
    fn test_function_body_expr_case() {
        assert_eq!(
            function_body("= name"),
            Ok((
                "",
                FunBody::Expr(Expr::Name(NameExprSyntax {
                    name_space: Default::default(),
                    name: TokenSyntax::from("name")
                }))
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
                    modifiers: Default::default(),
                    fun_keyword: TokenSyntax::from("fun"),
                    name: TokenSyntax::from("function")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    type_params: None,
                    arg_defs: ArgDefListSyntax::default(),
                    return_type: None,
                    type_constraints: None,
                    body: Some(FunBody::Block(BlockSyntax {
                        open: TokenSyntax::from("{"),
                        body: vec![],
                        close: TokenSyntax::from("}")
                    })),
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
                    modifiers: Default::default(),
                    fun_keyword: TokenSyntax::from("fun"),
                    name: TokenSyntax::from("puts")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    type_params: None,
                    arg_defs: ArgDefListSyntax {
                        open: TokenSyntax::from("("),
                        elements: vec![ArgDefElementSyntax {
                            element: ArgDef::Value(ValueArgDef {
                                label: Some(
                                    TokenSyntax::from("_")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                ),
                                name: TokenSyntax::from("item"),
                                type_name: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("String"),
                                    type_args: None
                                })
                            }),
                            trailing_comma: None
                        }],
                        close: TokenSyntax::from(")")
                    },
                    return_type: Some(TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Unit"),
                        type_args: None
                    })),
                    type_constraints: None,
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
                    modifiers: Default::default(),
                    fun_keyword: TokenSyntax::from("fun"),
                    name: TokenSyntax::from("puts")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    type_params: None,
                    arg_defs: ArgDefListSyntax {
                        open: TokenSyntax::from("("),
                        elements: vec![ArgDefElementSyntax {
                            element: ArgDef::Value(ValueArgDef {
                                label: None,
                                name: TokenSyntax::from("item"),
                                type_name: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("String"),
                                    type_args: None
                                })
                            }),
                            trailing_comma: None
                        }],
                        close: TokenSyntax::from(")")
                    },
                    return_type: Some(TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Unit"),
                        type_args: None
                    })),
                    type_constraints: None,
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
                    mutability_keyword: TokenSyntax::from("val")
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    name: TokenSyntax::from("a"),
                    type_: Some(TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None
                    })),
                    value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
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
                    mutability_keyword: TokenSyntax::from("val")
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    name: TokenSyntax::from("a"),
                    type_: None,
                    value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
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
                    name: TokenSyntax::from("T"),
                    type_constraint: Some(TypeConstraintSyntax {
                        sep: TokenSyntax::from(":")
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        constraint: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Printable"),
                            type_args: None
                        })
                    })
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
                TypeConstraintsSyntax {
                    where_keyword: TokenSyntax::from("where"),
                    type_constraints: vec![TypeConstraintElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":")
                                    .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("Printable"),
                                    type_args: None
                                })
                            })
                        },
                        trailing_comma: Some(TokenSyntax::from(","))
                    }]
                }
            ))
        );
        assert_eq!(
            type_constraints("where T: Printable, T: DebugPrintable"),
            Ok((
                "",
                TypeConstraintsSyntax {
                    where_keyword: TokenSyntax::from("where"),
                    type_constraints: vec![
                        TypeConstraintElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("Printable"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(TokenSyntax::from(","))
                        },
                        TypeConstraintElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("DebugPrintable"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: None
                        }
                    ]
                }
            ))
        );
        assert_eq!(
            type_constraints("where T: Printable, T: DebugPrintable,"),
            Ok((
                "",
                TypeConstraintsSyntax {
                    where_keyword: TokenSyntax::from("where"),
                    type_constraints: vec![
                        TypeConstraintElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("Printable"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(TokenSyntax::from(","))
                        },
                        TypeConstraintElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("DebugPrintable"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(TokenSyntax::from(","))
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_package_name() {
        assert_eq!(
            package_name("abc::"),
            Ok((
                "",
                PackageName {
                    names: vec![PackageNameElement {
                        name: TokenSyntax::from("abc"),
                        sep: TokenSyntax::from("::")
                    }]
                }
            ))
        );
        assert_eq!(
            package_name("abc::def::"),
            Ok((
                "",
                PackageName {
                    names: vec![
                        PackageNameElement {
                            name: TokenSyntax::from("abc"),
                            sep: TokenSyntax::from("::")
                        },
                        PackageNameElement {
                            name: TokenSyntax::from("def"),
                            sep: TokenSyntax::from("::")
                        }
                    ]
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
                    use_keyword: TokenSyntax::from("use"),
                    package_name: PackageName { names: vec![] },
                    used_name: TokenSyntax::from("abc"),
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
                    use_keyword: TokenSyntax::from("use"),
                    package_name: PackageName { names: vec![] },
                    used_name: TokenSyntax::from("abc"),
                    alias: Some(AliasSyntax {
                        as_keyword: TokenSyntax::from("as")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        name: TokenSyntax::from("def")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    })
                }
            ))
        );
    }
}
