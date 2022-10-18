use crate::parser::error::ParseError;
use crate::parser::wiz::statement::file;
use crate::parser::Span;
use std::collections::BTreeMap;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;
use wiz_result::Result;
use wiz_session::ParseSession;
use wiz_span::{get_line_offset, Location};
use wiz_syntax::syntax::declaration::{DeclKind, DeclarationSyntax};
use wiz_syntax::syntax::{FileSyntax, WizFile};

pub mod annotation;
pub mod character;
pub mod declaration;
pub mod expression;
pub mod keywords;
pub mod lexical_structure;
pub mod name_space;
pub mod operators;
pub mod statement;
pub mod type_;

pub fn parse_from_string<P: AsRef<Path>>(
    session: &ParseSession,
    src_path: Option<P>,
    src: &str,
    name: Option<&str>,
) -> Result<WizFile> {
    match file(session, Span::from(src)) {
        Ok((s, f)) => {
            if !s.is_empty() {
                let location = Location::new(s.location_offset(), s.location_line());
                Err(Box::new(ParseError::from(get_error_location_src(
                    src_path, src, &location,
                ))))
            } else {
                Ok(WizFile {
                    name: name.unwrap_or_default().to_string(),
                    syntax: f,
                })
            }
        }
        Err(_) => Err(Box::new(ParseError::from(String::new()))),
    }
}

pub fn parse_from_file_path<P: AsRef<Path>>(
    session: &ParseSession,
    path: P,
    name: Option<&str>,
) -> Result<WizFile> {
    let s = read_to_string(&path)?;
    parse_from_string(
        session,
        Some(&path),
        &*s,
        name.or_else(|| path.as_ref().file_stem().and_then(|p| p.to_str())),
    )
}

pub fn read_package_from_path(
    session: &ParseSession,
    path: &Path,
    name: Option<&str>,
) -> Result<WizFile> {
    Ok(if path.is_dir() {
        let dir = fs::read_dir(path)?;
        WizFile {
            name: name
                .or_else(|| path.file_stem().and_then(|p| p.to_str()))
                .unwrap_or_default()
                .to_string(),
            syntax: FileSyntax {
                leading_trivia: Default::default(),
                body: dir
                    .into_iter()
                    .map(|d| read_package_from_path(session, &*d?.path(), None))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .fold(BTreeMap::new(), |mut acc, value| {
                        acc.entry(value.name.to_string())
                            .or_insert_with(Vec::new)
                            .push(value);
                        acc
                    })
                    .into_iter()
                    .map(|(k, mut v)| {
                        if v.len() == 1 {
                            v.remove(0)
                        } else {
                            let mut sb = WizFile {
                                name: k,
                                syntax: FileSyntax {
                                    leading_trivia: Default::default(),
                                    body: Default::default(),
                                    trailing_trivia: Default::default(),
                                },
                            };
                            for item in v.into_iter() {
                                sb.syntax.body.extend(item.syntax.body);
                            }
                            sb
                        }
                    })
                    .map(|i| {
                        let WizFile { name, syntax } = i;
                        DeclarationSyntax {
                            annotations: None,
                            kind: DeclKind::Module((name, Some(syntax))),
                        }
                    })
                    .collect(),
                trailing_trivia: Default::default(),
            },
        }
    } else {
        parse_from_file_path(session, path, name)?
    })
}

fn get_error_location_src<P: AsRef<Path>>(
    src_path: Option<P>,
    src: &str,
    location: &Location,
) -> String {
    let line_offset = get_line_offset(src, location);
    let error_line = src
        .lines()
        .nth(location.line() as usize - 1)
        .unwrap_or_default();
    format!(
        "{}:L{} | {}\n{}^",
        src_path.map_or_else(
            || String::from("Unknown source"),
            |it| it.as_ref().display().to_string(),
        ),
        location.line(),
        error_line,
        " ".repeat(location.line().to_string().len() + 3 + line_offset)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_string() {
        let session = ParseSession::default();
        let result = parse_from_string::<&str>(&session, None, "unknown_token", None);
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Unknown source:L1 | unknown_token\n    ^");
        } else {
            unreachable!();
        }
    }
}
