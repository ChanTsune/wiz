use crate::constant::MANIFEST_FILE_NAME;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod error;
pub mod workspace;

pub(crate) fn create_project(path: &Path, project_name: &str) -> Result<(), Box<dyn Error>> {
    let mut path = path.to_path_buf();
    path.push(MANIFEST_FILE_NAME);
    let mut package_wiz = BufWriter::new(File::create(&path)?);
    writeln!(
        package_wiz,
        r#"
[package]
name = {:?}
version = "0.1.0"

[dependencies]
core = "0.0.0"
std = "0.0.0"
"#,
        project_name
    )?;
    path.pop();

    path.push("src");
    create_dir_all(&path)?;
    path.push("main.wiz");
    let mut main_wiz = BufWriter::new(File::create(&path)?);
    writeln!(
        main_wiz,
        r#"
fun main() {{
    println("Hello world!")
}}"#
    )?;
    Ok(())
}
