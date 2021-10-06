use crate::parser::error::ParseError;
use crate::parser::result::Result;
use crate::parser::wiz::statement::file;
use crate::parser::{Location, Span};
use crate::syntax::file::{SourceSet, WizFile};
use std::fs;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;

pub mod annotation;
pub mod character;
pub mod declaration;
pub mod expression;
pub mod keywords;
pub mod lexical_structure;
pub mod operators;
pub mod statement;
pub mod type_;

pub fn parse_from_string(string: &str) -> Result<WizFile> {
    return match file(Span::from(string)) {
        Ok((s, f)) => {
            if !s.is_empty() {
                return Result::Err(ParseError::ParseError(String::from(format!(
                    "{:?}{}",
                    Location::from(s),
                    s
                ))));
            }
            Result::Ok(WizFile {
                name: String::new(),
                syntax: f,
            })
        }
        Err(_) => Result::Err(ParseError::from(String::new())),
    };
}

pub fn parse_from_file(mut file: File) -> Result<WizFile> {
    let mut string = String::new();
    let _ = file.read_to_string(&mut string)?;
    parse_from_string(&*string)
}

pub fn parse_from_file_path_str(path: &str) -> Result<WizFile> {
    let p = Path::new(path);
    parse_from_file_path(p)
}

pub fn parse_from_file_path(path: &Path) -> Result<WizFile> {
    let s = read_to_string(path)?;
    let mut f = parse_from_string(&*s)?;
    f.name = String::from(path.to_string_lossy());
    Ok(f)
}

pub fn read_package_from_path(path: &Path) -> Result<SourceSet> {
    if !path.is_dir() {
        Result::Err(ParseError::ParseError(String::from(format!(
            "{:?} is not package dir",
            path
        ))))
    } else {
        let dir = fs::read_dir(path)?;
        for item in dir.into_iter() {
            let dir_entry = item.unwrap();
            if dir_entry.file_name().to_str().unwrap() == "src" {
                return Result::Ok(SourceSet::Dir {
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    items: match read_package_files(dir_entry.path().as_path())? {
                        SourceSet::File(_) => {
                            panic!("never execution branch executed!!")
                        }
                        SourceSet::Dir { name: _, items } => items,
                    },
                });
            }
            println!("{}", dir_entry.path().to_str().unwrap());
        }
        Result::Ok(SourceSet::Dir {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            items: vec![],
        })
    }
}

fn read_package_files(path: &Path) -> Result<SourceSet> {
    if path.is_dir() {
        let dir = fs::read_dir(path)?;
        Result::Ok(SourceSet::Dir {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            items: dir
                .into_iter()
                .map(|d| read_package_files(&*d.unwrap().path()))
                .collect::<Result<Vec<SourceSet>>>()?,
        })
    } else {
        Result::Ok(SourceSet::File(parse_from_file_path(path)?))
    }
}
