use std::collections::BTreeMap;
use crate::constant::MANIFEST_FILE_NAME;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::core::manifest::{Manifest, PackageInfo};

pub mod error;
pub mod manifest;
pub mod workspace;

pub(crate) fn create_project(path: &Path, project_name: &str) -> Result<(), Box<dyn Error>> {
    let mut path = path.to_path_buf();
    path.push(MANIFEST_FILE_NAME);
    manifest::write(&path, &Manifest {
        package: PackageInfo { name: project_name.to_string(), version: "0.1.0".to_string() },
        dependencies: {
            let mut map = BTreeMap::new();
            map.insert("core".to_string(), "0.0.0".to_string());
            map.insert("std".to_string(), "0.0.0".to_string());
            map
        },
    })?;
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
