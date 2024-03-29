use crate::constant::MANIFEST_FILE_NAME;
use crate::core::manifest::{Dependencies, Dependency, Manifest, Package};
use crate::core::workspace::{construct_workspace_from, Workspace};
use clap::{ArgMatches, Command};
pub(crate) use context::WizContext;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

mod context;
pub mod dep;
pub mod error;
pub mod manifest;
pub mod workspace;

pub(crate) type Result<T> = wiz_result::Result<T>;
pub(crate) type Error = wiz_result::Error;

pub(crate) trait Cmd {
    const NAME: &'static str;
    fn command() -> Command {
        Command::new(Self::NAME)
    }
    fn execute(args: &ArgMatches) -> Result<()>;
}

pub(crate) fn create_project(path: &Path, project_name: &str) -> Result<()> {
    let manifest_path = path.join(MANIFEST_FILE_NAME);
    manifest::write(
        &manifest_path,
        &Manifest {
            package: Package::new(project_name, "0.1.0"),
            dependencies: Dependencies::from([
                ("core".to_string(), Dependency::simple("0.0.0")),
                ("std".to_string(), Dependency::simple("0.0.0")),
            ]),
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
        PathBuf::from(manifest_path)
    } else {
        env::current_dir()?.join(MANIFEST_FILE_NAME)
    };
    construct_workspace_from(&manifest_path)
}
