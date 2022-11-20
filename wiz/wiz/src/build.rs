use crate::core::dep::{resolve_manifest_dependencies, ResolvedDependencyTree};
use crate::core::error::CliError;
use crate::core::workspace::Workspace;
use crate::core::{Error, Result};
use crate::core::{load_project, Cmd};
use clap::ArgMatches;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use wiz_utils::topological_sort::topological_sort;
use wizc_cli::{BuildType, Config, ConfigBuilder};
use wizc_message::MessageParser;

pub(crate) struct BuildCommand;

impl Cmd for BuildCommand {
    const NAME: &'static str = "build";

    fn execute(args: &ArgMatches) -> Result<()> {
        command(Self::NAME, Options::from(args))?;
        Ok(())
    }
}

pub(crate) struct Options<'ops> {
    manifest_path: Option<&'ops str>,
    std: Option<&'ops str>,
    target_dir: Option<&'ops str>,
    target_triple: Option<&'ops str>,
    test: bool,
}

impl<'ops> Options<'ops> {
    pub(crate) fn new(
        manifest_path: Option<&'ops str>,
        std: Option<&'ops str>,
        target_dir: Option<&'ops str>,
        target_triple: Option<&'ops str>,
        test: bool,
    ) -> Self {
        Self {
            manifest_path,
            std,
            target_dir,
            target_triple,
            test,
        }
    }
}

impl<'ops> From<&'ops ArgMatches> for Options<'ops> {
    fn from(args: &'ops ArgMatches) -> Self {
        Self::new(
            args.get_one::<String>("manifest-path").map(|i| i.as_str()),
            args.get_one::<String>("std").map(|i| i.as_str()),
            args.get_one::<String>("target-dir").map(|i| i.as_str()),
            args.get_one::<String>("target-triple").map(|i| i.as_str()),
            args.get_flag("tests"),
        )
    }
}

pub(crate) fn command(_: &str, options: Options) -> Result<()> {
    let ws = load_project(options.manifest_path)?;

    let resolved_dependencies =
        resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?, options.std)?;

    println!("{}", resolved_dependencies);

    let target_dir = if let Some(target_dir) = options.target_dir {
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

    let wlib_paths = compile_dependencies(&ws, resolved_dependencies, &target_dir)?;

    let input_path = {
        let src_dir = ws.cws.join("src");
        let main_file = src_dir.join("main.wiz");
        if main_file.exists() {
            main_file
        } else {
            src_dir.join("lib.wiz")
        }
    };
    let mut config = Config::default()
        .input(input_path)
        .out_dir(target_dir)
        .name(ws.cws.file_name().and_then(OsStr::to_str).unwrap())
        .type_(if options.test {
            BuildType::Test
        } else {
            BuildType::Binary
        })
        .libraries(&wlib_paths.iter().collect::<Vec<_>>());

    config = if let Some(target_triple) = options.target_triple {
        config.target_triple(target_triple)
    } else {
        config
    };

    super::subcommand::execute("wizc", config.as_args())
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Task {
    name: String,
    version: String,
    src_path: PathBuf,
}

fn dependency_list(dependencies: ResolvedDependencyTree) -> HashMap<Task, HashSet<Task>> {
    fn dependency_list(
        result: &mut HashMap<Task, HashSet<Task>>,
        dep: ResolvedDependencyTree,
    ) -> Task {
        let ResolvedDependencyTree {
            name,
            version,
            src_path,
            dependencies,
        } = dep;
        let task = Task {
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
    target_dir: &Path,
) -> Result<BTreeSet<PathBuf>> {
    let message_parser = MessageParser::new();
    let mut wlib_paths = BTreeSet::new();
    let dependen_list = dependency_list(dependencies);
    let dep_list = topological_sort(dependen_list.clone())?;
    for dep in dep_list.into_iter().flatten() {
        let dep_wlib_paths = dependen_list
            .get(&dep)
            .unwrap()
            .iter()
            .map(|d| {
                let mut path = target_dir.join(&d.name);
                path.set_extension("wlib");
                path
            })
            .collect::<Vec<_>>();
        let input = dep.src_path.to_string_lossy().to_string();
        let output = super::subcommand::output(
            "wizc",
            Config::default()
                .input(&input)
                .out_dir(target_dir)
                .name(&dep.name)
                .type_(BuildType::Library)
                .libraries(&dep_wlib_paths)
                .as_args(),
        )?;
        for line in String::from_utf8_lossy(&output.stdout).split_terminator('\n') {
            match message_parser.parse(line) {
                Ok(message) => println!("{}", message),
                Err(_) => println!("{}", line),
            }
        }
        if !output.stderr.is_empty() {
            for line in String::from_utf8_lossy(&output.stderr).split_terminator('\n') {
                match message_parser.parse(line) {
                    Ok(message) => eprintln!("{}", message),
                    Err(_) => eprintln!("{}", line),
                }
            }
        }
        if !output.status.success() {
            return Err(Box::new(CliError::from(format!(
                "compile failed {:?}",
                dep.name
            ))));
        }
        wlib_paths.extend(dep_wlib_paths);
        wlib_paths.insert({
            let mut path = target_dir.join(dep.name);
            path.set_extension("wlib");
            path
        });
    }
    Ok(wlib_paths)
}
