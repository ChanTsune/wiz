use crate::core::dep::{resolve_manifest_dependencies, ResolvedDependencyTree};
use crate::core::error::CliError;
use crate::core::load_project;
use crate::core::workspace::Workspace;
use clap::ArgMatches;
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub(crate) const COMMAND_NAME: &str = "build";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = options.value_of("manifest-path");

    let ws = load_project(manifest_path)?;

    let resolved_dependencies = resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?)?;

    println!("{:?}", resolved_dependencies);

    let target_dir = if let Some(target_dir) = options.value_of("target-dir") {
        let d = PathBuf::from(target_dir);
        if d.exists() && !d.is_dir() {
            return Err(Box::from(CliError::from(format!(
                "{} is not directory",
                d.display()
            ))));
        } else {
            d
        }
    } else {
        env::current_dir()?.join("target")
    };
    create_dir_all(&target_dir)?;

    let wlib_paths = compile_dependencies(
        &ws,
        &resolved_dependencies.dependencies,
        target_dir.to_str().unwrap(),
    )?;

    let mut args = vec![ws.cws.to_str().unwrap()];
    args.extend(["--out-dir", target_dir.to_str().unwrap()]);

    args.extend(["--name", ws.cws.file_name().unwrap().to_str().unwrap()]);
    args.extend(["--type", "bin"]);

    for wlib_path in wlib_paths.iter() {
        args.extend(["--library", wlib_path]);
    }

    if let Some(target_triple) = options.value_of("target-triple") {
        args.extend(["--target-triple", target_triple]);
    };

    super::subcommand::execute("wizc", &args)
}

fn compile_dependencies(
    ws: &Workspace,
    dependencies: &[ResolvedDependencyTree],
    target_dir: &str,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut wlib_paths = BTreeSet::new();
    for dep in dependencies {
        let dep_wlib_paths = compile_dependencies(ws, &dep.dependencies, target_dir)?;
        let mut args = vec![dep.src_path.as_str()];
        args.extend(["--out-dir", target_dir]);
        args.extend(["--name", dep.name.as_str()]);
        args.extend(["--type", "lib"]);
        for wlib_path in dep_wlib_paths.iter() {
            args.extend(["--library", wlib_path]);
        }
        let output = super::subcommand::output("wizc", &args)?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        wlib_paths.extend(dep_wlib_paths);
        wlib_paths.insert(format!("{}/{}.wlib", target_dir, dep.name));
    }
    Ok(wlib_paths)
}
