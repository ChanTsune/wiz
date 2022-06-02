use crate::core::dep::{resolve_manifest_dependencies, ResolvedDependencyTree};
use crate::core::error::CliError;
use crate::core::load_project;
use crate::core::workspace::Workspace;
use clap::ArgMatches;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
use wiz_utils::topological_sort::topological_sort;

pub(crate) const COMMAND_NAME: &str = "build";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = options.value_of("manifest-path");

    let another_std = options.value_of("std");

    let ws = load_project(manifest_path)?;

    let resolved_dependencies =
        resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?, another_std)?;

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

    let wlib_paths =
        compile_dependencies(&ws, resolved_dependencies, target_dir.to_str().unwrap())?;

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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct SimpleDep {
    name: String,
    version: String,
    src_path: String,
}

fn dependency_list(dependencies: ResolvedDependencyTree) -> HashMap<SimpleDep, HashSet<SimpleDep>> {
    fn dependency_list(
        result: &mut HashMap<SimpleDep, HashSet<SimpleDep>>,
        dep: ResolvedDependencyTree,
    ) -> SimpleDep {
        let ResolvedDependencyTree {
            name,
            version,
            src_path,
            dependencies,
        } = dep;
        let task = SimpleDep {
            name,
            version,
            src_path,
        };
        let dependencies = dependencies
            .into_iter()
            .map(|d| dependency_list(result, d))
            .collect();
        result.insert(task.clone(), dependencies);
        task
    }
    let mut result = HashMap::new();
    for dependency in dependencies.dependencies {
        dependency_list(&mut result, dependency);
    }
    result
}

fn compile_dependencies(
    ws: &Workspace,
    dependencies: ResolvedDependencyTree,
    target_dir: &str,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut wlib_paths = BTreeSet::new();
    let dependen_list = dependency_list(dependencies);
    let dep_list = topological_sort(dependen_list.clone())?;
    for dep in dep_list.into_iter().flatten() {
        let dep_wlib_paths = dependen_list
            .get(&dep)
            .unwrap()
            .iter()
            .map(|d| format!("{}/{}.wlib", target_dir, d.name))
            .collect::<Vec<_>>();
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
        if !output.status.success() {
            return Err(Box::new(CliError::from(format!(
                "compile failed {:?}",
                dep.name
            ))));
        }
        wlib_paths.extend(dep_wlib_paths);
        wlib_paths.insert(format!("{}/{}.wlib", target_dir, dep.name));
    }
    Ok(wlib_paths)
}
