use crate::parser::error::ParseError;
use crate::parser::result::Result;
use crate::parser::wiz::statement::file;
use crate::parser::Span;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;
use wiz_span::{get_line_offset, Location};
use wiz_syntax::syntax::file::{SourceSet, WizFile};

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

pub fn parse_from_string(src: &str, name: Option<&str>) -> Result<WizFile> {
    match file(Span::from(src)) {
        Ok((s, f)) => {
            if !s.is_empty() {
                let location = Location::new(s.location_offset(), s.location_line());
                Err(ParseError::from(get_error_location_src(src, &location)))
            } else {
                Ok(WizFile {
                    name: name.unwrap_or_default().to_string(),
                    syntax: f,
                })
            }
        }
        Err(_) => Err(ParseError::from(String::new())),
    }
}

pub fn parse_from_file_path_str(path: &str) -> Result<WizFile> {
    let p = Path::new(path);
    parse_from_file_path(p)
}

pub fn parse_from_file_path(path: &Path) -> Result<WizFile> {
    let s = read_to_string(path)?;
    parse_from_string(&*s, path.as_os_str().to_str())
}

pub fn read_package_from_path(path: &Path, name: Option<&str>) -> Result<SourceSet> {
    let dir = fs::read_dir(path)?;
    for item in dir {
        let dir_entry = item.unwrap();
        if let Some("src") = dir_entry.file_name().to_str() {
            return Ok(SourceSet::Dir {
                name: name
                    .or_else(|| path.file_name().and_then(|p| p.to_str()))
                    .unwrap_or_default()
                    .to_string(),
                items: match read_package_files(dir_entry.path().as_path())? {
                    SourceSet::File(_) => unreachable!(),
                    SourceSet::Dir { name: _, items } => items,
                },
            });
        }
        println!("{}", dir_entry.path().display());
    }
    Ok(SourceSet::Dir {
        name: path
            .file_name()
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string(),
        items: vec![],
    })
}

fn read_package_files(path: &Path) -> Result<SourceSet> {
    Ok(if path.is_dir() {
        let dir = fs::read_dir(path)?;
        SourceSet::Dir {
            name: path
                .file_name()
                .and_then(|p| p.to_str())
                .unwrap()
                .to_string(),
            items: dir
                .into_iter()
                .map(|d| read_package_files(&*d.unwrap().path()))
                .collect::<Result<_>>()?,
        }
    } else {
        SourceSet::File(parse_from_file_path(path)?)
    })
}

fn get_error_location_src(src: &str, location: &Location) -> String {
    let line_offset = get_line_offset(src, location);
    let error_line = src
        .lines()
        .nth(location.line() as usize - 1)
        .unwrap_or_default();
    format!(
        "{} | {}\n{}^",
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
        let result = parse_from_string("unknown_token", None);
        if let Err(e) = result {
            assert_eq!(e.to_string(), "1 | unknown_token\n    ^");
        } else {
            unreachable!();
        }
    }
}
