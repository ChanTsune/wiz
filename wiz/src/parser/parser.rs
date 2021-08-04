use crate::ast::file::WizFile;
use crate::parser::nom::file;
use std::fs::{File, read_to_string};
use std::io;
use std::io::Read;
use std::process::exit;
use std::path::Path;

pub fn parse_from_string(string: String) -> WizFile {
    return match file(&*string) {
        Ok((s, f)) => {
            if !s.is_empty() {
                eprintln!("{}", s);
                exit(-1)
            }
            WizFile {
                name: String::new(),
                syntax: f,
            }
        }
        Err(..) => exit(-1),
    };
}

pub fn parse_from_file(mut file: File) -> io::Result<WizFile> {
    let mut string = String::new();
    let result = file.read_to_string(&mut string);
    return result.map(|_| parse_from_string(string));
}

pub fn parse_from_file_path_str(path: &str) -> io::Result<WizFile> {
    let p = Path::new(path);
    parse_from_file_path(p)
}

pub fn parse_from_file_path(path: &Path) -> io::Result<WizFile> {
    let s = read_to_string(path)?;
    let mut f = parse_from_string(s);
    f.name = String::from(path.to_string_lossy());
    Ok(f)
}