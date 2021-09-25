use crate::parser::error::ParseError;
use crate::parser::wiz::file;
use crate::syntax::file::WizFile;
use crate::parser::result::Result;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;


pub fn parse_from_string(string: String) -> Result<WizFile> {
    return match file(&*string) {
        Ok((s, f)) => {
            if !s.is_empty() {
                return Result::Err(ParseError::ParseError(String::from(format!("{}", s))));
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
    parse_from_string(string)
}

pub fn parse_from_file_path_str(path: &str) -> Result<WizFile> {
    let p = Path::new(path);
    parse_from_file_path(p)
}

pub fn parse_from_file_path(path: &Path) -> Result<WizFile> {
    let s = read_to_string(path)?;
    let mut f = parse_from_string(s)?;
    f.name = String::from(path.to_string_lossy());
    Ok(f)
}
