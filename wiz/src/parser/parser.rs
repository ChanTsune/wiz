use crate::ast::file::WizFile;
use crate::parser::nom::file;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;

pub fn parse_from_string(string: String) -> WizFile {
    return match file(&*string) {
        Ok((s, f)) => WizFile {
            name: String::new(),
            syntax: f,
        },
        Err(..) => exit(-1),
    };
}

pub fn parse_from_file(mut file: File) -> io::Result<WizFile> {
    let mut string = String::new();
    let result = file.read_to_string(&mut string);
    return result.map(|_| parse_from_string(string));
}
