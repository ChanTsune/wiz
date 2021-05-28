use std::fs::File;
use std::io::Read;
use std::io;

fn parse_from_string(string: String) -> crate::ast::file::File {
    return crate::ast::file::File{ body: vec![] }
}

fn parse_from_file(mut file: File) -> io::Result<crate::ast::file::File> {
    let mut string = String::new();
    let result = file.read_to_string(&mut string);
    return result.map(|_| parse_from_string(string));
}
