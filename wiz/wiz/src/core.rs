use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod error;
pub mod workspace;

pub(crate) fn create_project(path: &Path, project_name: &str) -> Result<(), Box<dyn Error>> {
    let mut path = path.to_path_buf();
    path.push("Package.wiz");
    let mut package_wiz = BufWriter::new(File::create(&path)?);
    writeln!(
        package_wiz,
        r#"
val package = Package.init(
    name: {:?},
)"#,
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
