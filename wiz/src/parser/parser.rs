use crate::parser::nom::file;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;

pub fn parse_from_string(string: String) -> crate::ast::file::FileSyntax {
    return match file(&*string) {
        Ok((s, f)) => f,
        Err(..) => exit(-1),
    };
}

pub fn parse_from_file(mut file: File) -> io::Result<crate::ast::file::FileSyntax> {
    let mut string = String::new();
    let result = file.read_to_string(&mut string);
    return result.map(|_| parse_from_string(string));
}
