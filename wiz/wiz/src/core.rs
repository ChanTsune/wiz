use crate::constant::MANIFEST_FILE_NAME;
use crate::core::manifest::{Manifest, PackageInfo};
use crate::core::workspace::{construct_workspace_from, Workspace};
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use clap::ArgMatches;

pub mod dep;
pub mod error;
pub mod manifest;
pub mod workspace;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) trait Cmd {
    const NAME: &'static str;
    fn execute(args: &ArgMatches) -> Result<()>;
}

pub(crate) fn create_project(path: &Path, project_name: &str) -> Result<()> {
    let manifest_path = path.join(MANIFEST_FILE_NAME);
    manifest::write(
        &manifest_path,
        &Manifest {
            package: PackageInfo {
                name: project_name.to_string(),
                version: "0.1.0".to_string(),
            },
            dependencies: {
                let mut map = BTreeMap::new();
                map.insert("core".to_string(), "0.0.0".to_string());
                map.insert("std".to_string(), "0.0.0".to_string());
                map
            },
        },
    )?;

    let src_dir = path.join("src");
    create_dir_all(&src_dir)?;

    let main_wiz_path = src_dir.join("main.wiz");
    let mut main_wiz = BufWriter::new(File::create(&main_wiz_path)?);
    writeln!(
        main_wiz,
        r#"
fun main() {{
    println("Hello world!")
}}"#
    )?;
    Ok(())
}

pub(crate) fn load_project(path: Option<&str>) -> Result<Workspace> {
    let manifest_path = if let Some(manifest_path) = path {
        PathBuf::from(manifest_path).parent().unwrap().to_path_buf()
    } else {
        env::current_dir()?
    };
    construct_workspace_from(&manifest_path)
}
