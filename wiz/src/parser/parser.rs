use crate::parser::error::ParseError;
use crate::parser::result::Result;
use crate::parser::wiz::statement::file;
use crate::syntax::file::WizFile;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;
