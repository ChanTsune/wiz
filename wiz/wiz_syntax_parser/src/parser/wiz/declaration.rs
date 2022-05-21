use crate::parser::wiz::annotation::annotations_syntax;
use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::expression::expr;
use crate::parser::wiz::keywords::{
    as_keyword, deinit_keyword, extension_keyword, fun_keyword, protocol_keyword, self_keyword,
    struct_keyword, use_keyword, val_keyword, var_keyword, where_keyword,
};
use crate::parser::wiz::lexical_structure::{identifier, token, whitespace0, whitespace1};
use crate::parser::wiz::statement::stmt;
use crate::parser::wiz::type_::{type_, type_parameter, type_parameters};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::{
    AsChar, Compare, ExtendInto, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use std::ops::{Range, RangeFrom};
use wiz_syntax::syntax::block::BlockSyntax;
use wiz_syntax::syntax::declaration::fun_syntax::{
    ArgDef, ArgDefElementSyntax, ArgDefListSyntax, ExprFunBodySyntax, FunBody, FunSyntax,
    SelfArgDefSyntax, ValueArgDef,
};
use wiz_syntax::syntax::declaration::{
    AliasSyntax, DeclKind, DeclarationSyntax, DeinitializerSyntax, ExtensionSyntax, PackageName,
    ProtocolConformSyntax, StoredPropertySyntax, StructBodySyntax, StructPropertySyntax,
    StructSyntax, TypeAnnotationSyntax, UseSyntax,
};
use wiz_syntax::syntax::declaration::{PackageNameElement, VarSyntax};
use wiz_syntax::syntax::token::TokenSyntax;
use wiz_syntax::syntax::type_name::{TypeConstraintElementSyntax, TypeConstraintsSyntax};
use wiz_syntax::syntax::Syntax;

pub fn decl<I>(s: I) -> IResult<I, DeclarationSyntax>
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
            opt(annotations_syntax),
            whitespace0,
            alt((
                use_decl,
                struct_decl,
                function_decl,
                var_decl,
                extension_decl,
            )),
        )),
        |(a, ws, d)| DeclarationSyntax {
            annotations: a,
            kind: d.with_leading_trivia(ws),
        },
    )(s)
}

pub fn type_annotation_syntax<I>(s: I) -> IResult<I, TypeAnnotationSyntax>
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
    map(tuple((token(":"), whitespace0, type_)), |(c, ws, t)| {
        TypeAnnotationSyntax {
            colon: c,
            type_: t.with_leading_trivia(ws),
        }
    })(s)
}

//region struct

pub fn struct_decl<I>(s: I) -> IResult<I, DeclKind>
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
    map(struct_syntax, DeclKind::Struct)(s)
}

// <struct_decl> ::= "struct" <identifier> <type_parameters>? <struct_body>
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
            opt(tuple((whitespace0, type_parameters))),
            opt(tuple((whitespace0, struct_body_syntax))),
        )),
        |(struct_keyword, nws, name, params, body)| match body {
            Some((ws, body)) => StructSyntax {
                struct_keyword,
                name: TokenSyntax::from(name).with_leading_trivia(nws),
                type_params: params.map(|(ws, p)| p.with_leading_trivia(ws)),
                body: body.with_leading_trivia(ws),
            },
            None => StructSyntax {
                struct_keyword,
                name: TokenSyntax::from(name).with_leading_trivia(nws),
                type_params: params.map(|(ws, p)| p.with_leading_trivia(ws)),
                body: Default::default(),
            },
        },
    )(s)
}

// <struct_body> ::= "{" <struct_properties> "}"
// <struct_properties> ::= (<struct_property> ("\n" <struct_property>)* "\n"?)?
pub fn struct_body_syntax<I>(s: I) -> IResult<I, StructBodySyntax>
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
            token("{"),
            opt(struct_property),
            many0(tuple((whitespace1, struct_property))),
            whitespace0,
            token("}"),
        )),
        |(open, property, properties, cws, close)| StructBodySyntax {
            open,
            properties: {
                let mut properties: Vec<_> = properties
                    .into_iter()
                    .map(|(ws, p)| p.with_leading_trivia(ws))
                    .collect();
                if let Some(p) = property {
                    properties.push(p);
                }
                properties
            },
            close: close.with_leading_trivia(cws),
        },
    )(s)
}

// <struct_property> ::= <stored_property>
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
    alt((stored_property, deinitializer, member_function))(s)
}

// <stored_property> ::= ("var" | "val") <identifier> ":" <type>
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
    map(stored_property_syntax, StructPropertySyntax::StoredProperty)(s)
}

// <stored_property> ::= ("var" | "val") <identifier> ":" <type>
pub fn stored_property_syntax<I>(s: I) -> IResult<I, StoredPropertySyntax>
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
            alt((var_keyword, val_keyword)),
            whitespace1,
            identifier,
            whitespace0,
            type_annotation_syntax,
        )),
        |(var, ws, name, tws, typ)| StoredPropertySyntax {
            mutability_keyword: var,
            name: TokenSyntax::from(name).with_leading_trivia(ws),
            type_: typ.with_leading_trivia(tws),
        },
    )(s)
}

// <deinitializer> =:: "deinit" <function_body>
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
        |(deinit, ws, body)| {
            StructPropertySyntax::Deinit(DeinitializerSyntax {
                deinit_keyword: deinit,
                body: body.with_leading_trivia(ws),
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

pub fn function_decl<I>(s: I) -> IResult<I, DeclKind>
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
    map(function_syntax, DeclKind::Fun)(s)
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
            opt(tuple((whitespace0, type_annotation_syntax))),
            opt(tuple((whitespace0, type_constraints))),
            opt(tuple((whitespace0, function_body))),
        )),
        |(f, ws, name, type_params, args, return_type, type_constraints, body)| FunSyntax {
            fun_keyword: f,
            name: TokenSyntax::from(name).with_leading_trivia(ws),
            type_params,
            arg_defs: args,
            return_type: return_type.map(|(ws, t)| t.with_leading_trivia(ws)),
            type_constraints: type_constraints.map(|(ws, c)| c.with_leading_trivia(ws)),
            body: body.map(|(ws, body)| body.with_leading_trivia(ws)),
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
            token("("),
            many0(tuple((
                whitespace0,
                function_value_parameter,
                whitespace0,
                comma,
            ))),
            opt(tuple((whitespace0, function_value_parameter))),
            whitespace0,
            token(")"),
        )),
        |(open, elements, element, tws, close)| {
            let mut elements: Vec<_> = elements
                .into_iter()
                .map(|(lws, e, rws, c)| ArgDefElementSyntax {
                    element: e.with_leading_trivia(lws),
                    trailing_comma: Some(c.with_leading_trivia(rws)),
                })
                .collect();
            if let Some((ws, e)) = element {
                elements.push(ArgDefElementSyntax {
                    element: e.with_leading_trivia(ws),
                    trailing_comma: None,
                });
            };

            ArgDefListSyntax {
                open,
                elements,
                close: close.with_leading_trivia(tws),
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
                opt(tuple((function_value_label, whitespace1))),
                function_value_name,
                whitespace0,
                token(":"),
                whitespace0,
                type_,
            )),
            |(label, name, cws, colon, ws, typ)| {
                ArgDef::Value(match label {
                    None => ValueArgDef {
                        label: None,
                        name: TokenSyntax::from(name),
                        colon: colon.with_leading_trivia(cws),
                        type_name: typ.with_leading_trivia(ws),
                    },
                    Some((label, lws)) => ValueArgDef {
                        label: Some(TokenSyntax::from(label)),
                        name: TokenSyntax::from(name).with_leading_trivia(lws),
                        colon: colon.with_leading_trivia(cws),
                        type_name: typ.with_leading_trivia(ws),
                    },
                })
            },
        ),
        map(
            tuple((opt(ampersand), whitespace0, self_keyword)),
            |(amp, ws, s)| {
                ArgDef::Self_(SelfArgDefSyntax {
                    reference: amp,
                    self_: s.with_leading_trivia(ws),
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
            many0(tuple((whitespace0, type_parameter, whitespace0, comma))),
            opt(tuple((whitespace0, type_parameter))),
        )),
        |(where_keyword, ws, t, ts)| {
            let mut type_constraints: Vec<_> = t
                .into_iter()
                .map(|(lws, ta, rws, c)| TypeConstraintElementSyntax {
                    element: ta.with_leading_trivia(lws),
                    trailing_comma: Some(c.with_leading_trivia(rws)),
                })
                .collect();
            if let Some((ws, ts)) = ts {
                type_constraints.push(TypeConstraintElementSyntax {
                    element: ts.with_leading_trivia(ws),
                    trailing_comma: None,
                })
            };
            if !type_constraints.is_empty() {
                let t = type_constraints.remove(0).with_leading_trivia(ws);
                type_constraints.insert(0, t);
            };
            TypeConstraintsSyntax {
                where_keyword,
                type_constraints,
            }
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
        map(tuple((token("="), whitespace0, expr)), |(equal, ws, ex)| {
            FunBody::Expr(ExprFunBodySyntax {
                equal,
                expr: ex.with_leading_trivia(ws),
            })
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
        tuple((
            token("{"),
            many0(tuple((whitespace0, stmt))),
            whitespace0,
            token("}"),
        )),
        |(open, stmts, cws, close)| BlockSyntax {
            open,
            body: stmts
                .into_iter()
                .map(|(ws, s)| s.with_leading_trivia(ws))
                .collect(),
            close: close.with_leading_trivia(cws),
        },
    )(s)
}

//endregion

//region var

pub fn var_decl<I>(s: I) -> IResult<I, DeclKind>
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
    map(var_syntax, DeclKind::Var)(s)
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
        tuple((
            alt((var_keyword, val_keyword)),
            whitespace1,
            identifier,
            opt(tuple((whitespace0, type_annotation_syntax))),
            whitespace0,
            token("="),
            whitespace0,
            expr,
        )),
        |(mutability_keyword, ws, name, t, elws, eq, erws, e)| VarSyntax {
            mutability_keyword,
            name: TokenSyntax::from(name).with_leading_trivia(ws),
            type_annotation: t.map(|(ws, t)| t.with_leading_trivia(ws)),
            equal: eq.with_leading_trivia(elws),
            value: e.with_leading_trivia(erws),
        },
    )(s)
}

//endregion

//region use
pub fn use_decl<I>(s: I) -> IResult<I, DeclKind>
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
    map(use_syntax, DeclKind::Use)(s)
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
            opt(package_name),
            alt((identifier, map(tag("*"), |i: I| i.to_string()))),
            opt(tuple((whitespace1, as_keyword, whitespace1, identifier))),
        )),
        |(u, ws, pkg, n, alias)| match pkg {
            None => UseSyntax {
                use_keyword: TokenSyntax::from(u),
                package_name: None,
                used_name: TokenSyntax::from(n).with_leading_trivia(ws),
                alias: alias.map(|(lws, a, rws, n)| AliasSyntax {
                    as_keyword: TokenSyntax::from(a).with_leading_trivia(lws),
                    name: TokenSyntax::from(n).with_leading_trivia(rws),
                }),
            },
            Some(pkg) => UseSyntax {
                use_keyword: TokenSyntax::from(u),
                package_name: Some(pkg.with_leading_trivia(ws)),
                used_name: TokenSyntax::from(n),
                alias: alias.map(|(lws, a, rws, n)| AliasSyntax {
                    as_keyword: TokenSyntax::from(a).with_leading_trivia(lws),
                    name: TokenSyntax::from(n).with_leading_trivia(rws),
                }),
            },
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
    map(many1(tuple((identifier, token("::")))), |i| PackageName {
        names: i
            .into_iter()
            .map(|(name, sep)| PackageNameElement {
                name: TokenSyntax::from(name),
                sep,
            })
            .collect(),
    })(s)
}

//endregion

//region extension
pub fn extension_decl<I>(s: I) -> IResult<I, DeclKind>
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
    map(extension_syntax, DeclKind::Extension)(s)
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
            opt(tuple((whitespace0, token(":"), whitespace0, type_))),
            opt(tuple((whitespace0, type_constraints))),
            whitespace0,
            struct_body_syntax,
        )),
        |(kw, tp, ws, n, protocol, tc, ws1, body)| ExtensionSyntax {
            extension_keyword: kw,
            type_params: tp.map(|(t, tp)| tp.with_leading_trivia(t)),
            name: n.with_leading_trivia(ws),
            protocol_extension: protocol.map(|(lws, colon, tws, typ)| ProtocolConformSyntax {
                colon: colon.with_leading_trivia(lws),
                protocol: typ.with_leading_trivia(tws),
            }),
            type_constraints: tc.map(|(t, tc)| tc.with_leading_trivia(t)),
            body: body.with_leading_trivia(ws1),
        },
    )(s)
}
//endregion

#[cfg(test)]
mod tests {
    use crate::parser::tests::check;
    use crate::parser::wiz::declaration::{
        block, function_body, function_decl, member_function, package_name, stored_property,
        struct_syntax, type_constraints, use_syntax, var_decl,
    };
    use wiz_syntax::syntax::block::BlockSyntax;
    use wiz_syntax::syntax::declaration::fun_syntax::{
        ArgDef, ArgDefElementSyntax, ArgDefListSyntax, ExprFunBodySyntax, FunBody, FunSyntax,
        ValueArgDef,
    };
    use wiz_syntax::syntax::declaration::{
        AliasSyntax, DeclKind, PackageName, StoredPropertySyntax, StructBodySyntax,
        StructPropertySyntax, StructSyntax, TypeAnnotationSyntax, UseSyntax,
    };
    use wiz_syntax::syntax::declaration::{PackageNameElement, VarSyntax};
    use wiz_syntax::syntax::expression::{BinaryOperationSyntax, Expr, NameExprSyntax};
    use wiz_syntax::syntax::literal::LiteralSyntax;
    use wiz_syntax::syntax::statement::Stmt;
    use wiz_syntax::syntax::token::TokenSyntax;
    use wiz_syntax::syntax::trivia::{Trivia, TriviaPiece};
    use wiz_syntax::syntax::type_name::{
        SimpleTypeName, TypeConstraintElementSyntax, TypeConstraintSyntax, TypeConstraintsSyntax,
        TypeName, TypeParam,
    };
    use wiz_syntax::syntax::Syntax;

    #[test]
    fn test_stored_property() {
        check(
            "val a: Int64",
            stored_property,
            StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                mutability_keyword: TokenSyntax::from("val"),
                name: TokenSyntax::from("a")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_: TypeAnnotationSyntax {
                    colon: TokenSyntax::from(":"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int64"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                },
            }),
        );
        check(
            "var a: Int64",
            stored_property,
            StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                mutability_keyword: TokenSyntax::from("var"),
                name: TokenSyntax::from("a")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_: TypeAnnotationSyntax {
                    colon: TokenSyntax::from(":"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int64"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                },
            }),
        );
    }

    #[test]
    fn test_struct_syntax() {
        check(
            r"struct A {var a: String}",
            struct_syntax,
            StructSyntax {
                struct_keyword: TokenSyntax::from("struct"),
                name: TokenSyntax::from("A")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_params: None,
                body: StructBodySyntax {
                    open: TokenSyntax::from("{")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    properties: vec![StructPropertySyntax::StoredProperty(StoredPropertySyntax {
                        mutability_keyword: TokenSyntax::from("var"),
                        name: TokenSyntax::from("a")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        type_: TypeAnnotationSyntax {
                            colon: TokenSyntax::from(":"),
                            type_: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("String"),
                                type_args: None,
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        },
                    })],
                    close: TokenSyntax::from("}"),
                },
            },
        );
    }

    #[test]
    fn test_member_function() {
        check(
            "fun function() {}",
            member_function,
            StructPropertySyntax::Method(FunSyntax {
                fun_keyword: TokenSyntax::from("fun"),
                name: TokenSyntax::from("function")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_params: None,
                arg_defs: ArgDefListSyntax::default(),
                return_type: None,
                type_constraints: None,
                body: Some(FunBody::Block(BlockSyntax {
                    open: TokenSyntax::from("{")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    body: vec![],
                    close: TokenSyntax::from("}"),
                })),
            }),
        );
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
        check(
            r"{
    1
}",
            block,
            BlockSyntax {
                open: TokenSyntax::from("{"),
                body: vec![
                    Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from(
                        "1",
                    ))))
                    .with_leading_trivia(Trivia::from(vec![
                        TriviaPiece::Newlines(1),
                        TriviaPiece::Spaces(4),
                    ])),
                ],
                close: TokenSyntax::from("}")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Newlines(1))),
            },
        );
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
        check(
            "= name",
            function_body,
            FunBody::Expr(ExprFunBodySyntax {
                equal: TokenSyntax::from("="),
                expr: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("name")))
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            }),
        )
    }

    #[test]
    fn test_function_decl() {
        check(
            "fun function() {}",
            function_decl,
            DeclKind::Fun(FunSyntax {
                fun_keyword: TokenSyntax::from("fun"),
                name: TokenSyntax::from("function")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_params: None,
                arg_defs: ArgDefListSyntax::default(),
                return_type: None,
                type_constraints: None,
                body: Some(FunBody::Block(BlockSyntax {
                    open: TokenSyntax::from("{")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    body: vec![],
                    close: TokenSyntax::from("}"),
                })),
            }),
        );
    }

    #[test]
    fn test_function_no_body() {
        check(
            "fun puts(_ item: String): Unit",
            function_decl,
            DeclKind::Fun(FunSyntax {
                fun_keyword: TokenSyntax::from("fun"),
                name: TokenSyntax::from("puts")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_params: None,
                arg_defs: ArgDefListSyntax {
                    open: TokenSyntax::from("("),
                    elements: vec![ArgDefElementSyntax {
                        element: ArgDef::Value(ValueArgDef {
                            label: Some(TokenSyntax::from("_")),
                            name: TokenSyntax::from("item")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            colon: TokenSyntax::from(":"),
                            type_name: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("String")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_args: None,
                            }),
                        }),
                        trailing_comma: None,
                    }],
                    close: TokenSyntax::from(")"),
                },
                return_type: Some(TypeAnnotationSyntax {
                    colon: TokenSyntax::from(":"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Unit"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
                type_constraints: None,
                body: None,
            }),
        );
    }

    #[test]
    fn test_function_short_label() {
        check(
            "fun puts(item: String): Unit",
            function_decl,
            DeclKind::Fun(FunSyntax {
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
                            colon: TokenSyntax::from(":"),
                            type_name: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("String")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                type_args: None,
                            }),
                        }),
                        trailing_comma: None,
                    }],
                    close: TokenSyntax::from(")"),
                },
                return_type: Some(TypeAnnotationSyntax {
                    colon: TokenSyntax::from(":"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Unit"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
                type_constraints: None,
                body: None,
            }),
        );
    }

    #[test]
    fn test_var_decl() {
        check(
            "val a: Int = 1",
            var_decl,
            DeclKind::Var(VarSyntax {
                mutability_keyword: TokenSyntax::from("val"),
                name: TokenSyntax::from("a")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_annotation: Some(TypeAnnotationSyntax {
                    colon: TokenSyntax::from(":"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
                equal: TokenSyntax::from("=")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            }),
        );
    }

    #[test]
    fn test_var_decl_without_type() {
        check(
            "val a = 1",
            var_decl,
            DeclKind::Var(VarSyntax {
                mutability_keyword: TokenSyntax::from("val"),
                name: TokenSyntax::from("a")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                type_annotation: None,
                equal: TokenSyntax::from("=")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            }),
        );
    }

    #[test]
    fn test_type_constraints() {
        check(
            "where T: Printable,",
            type_constraints,
            TypeConstraintsSyntax {
                where_keyword: TokenSyntax::from("where"),
                type_constraints: vec![TypeConstraintElementSyntax {
                    element: TypeParam {
                        name: TokenSyntax::from("T")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        type_constraint: Some(TypeConstraintSyntax {
                            sep: TokenSyntax::from(":"),
                            constraint: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("Printable"),
                                type_args: None,
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        }),
                    },
                    trailing_comma: Some(TokenSyntax::from(",")),
                }],
            },
        );
        check(
            "where T: Printable, T: DebugPrintable",
            type_constraints,
            TypeConstraintsSyntax {
                where_keyword: TokenSyntax::from("where"),
                type_constraints: vec![
                    TypeConstraintElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("Printable"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: Some(TokenSyntax::from(",")),
                    },
                    TypeConstraintElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("DebugPrintable"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: None,
                    },
                ],
            },
        );
        check(
            "where T: Printable, T: DebugPrintable,",
            type_constraints,
            TypeConstraintsSyntax {
                where_keyword: TokenSyntax::from("where"),
                type_constraints: vec![
                    TypeConstraintElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("Printable"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: Some(TokenSyntax::from(",")),
                    },
                    TypeConstraintElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("DebugPrintable"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: Some(TokenSyntax::from(",")),
                    },
                ],
            },
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
        check(
            "use abc",
            use_syntax,
            UseSyntax {
                use_keyword: TokenSyntax::from("use"),
                package_name: None,
                used_name: TokenSyntax::from("abc")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                alias: None,
            },
        );
        check(
            "use abc as def",
            use_syntax,
            UseSyntax {
                use_keyword: TokenSyntax::from("use"),
                package_name: None,
                used_name: TokenSyntax::from("abc")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                alias: Some(AliasSyntax {
                    as_keyword: TokenSyntax::from("as")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    name: TokenSyntax::from("def")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
            },
        );
    }
}
