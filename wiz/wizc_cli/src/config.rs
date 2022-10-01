pub(crate) mod build_type;

use crate::config::build_type::BuildType;
use clap::ArgMatches;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct Config {
    input: String,
    name: Option<String>,
    type_: Option<BuildType>,
    output: Option<String>,
    out_dir: Option<String>,
    paths: Vec<String>,
    l: Option<String>,
    target_triple: Option<String>,
    libraries: Vec<String>,
    emit: Option<String>,
}

pub trait ConfigExt {
    fn input(&self) -> &Path;
    fn name(&self) -> Option<String>;
    fn type_(&self) -> BuildType;
    fn output(&self) -> Option<String>;
    fn out_dir(&self) -> Option<String>;
    fn paths(&self) -> Vec<PathBuf>;
    fn target_triple(&self) -> Option<String>;
    fn libraries(&self) -> Vec<PathBuf>;
    fn emit(&self) -> Option<String>;
}

impl ConfigExt for Config {
    fn input(&self) -> &Path {
        Path::new(&self.input)
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn type_(&self) -> BuildType {
        self.type_.unwrap_or(BuildType::Binary)
    }

    fn output(&self) -> Option<String> {
        self.output.clone()
    }

    fn out_dir(&self) -> Option<String> {
        self.out_dir.clone()
    }

    fn paths(&self) -> Vec<PathBuf> {
        self.paths.iter().map(PathBuf::from).collect()
    }

    fn target_triple(&self) -> Option<String> {
        self.target_triple.clone()
    }

    fn libraries(&self) -> Vec<PathBuf> {
        self.libraries.iter().map(PathBuf::from).collect()
    }

    fn emit(&self) -> Option<String> {
        self.emit.clone()
    }
}

pub trait ConfigBuilder {
    fn input(self, input: &str) -> Self;
    fn name(self, name: &str) -> Self;
    fn type_(self, build_type: BuildType) -> Self;
    fn output(self, output: &str) -> Self;
    fn out_dir(self, out_dir: &str) -> Self;
    fn path(self, path: &str) -> Self;
    fn paths(self, paths: &[&str]) -> Self;
    fn target_triple(self, target_triple: &str) -> Self;
    fn library(self, library: &str) -> Self;
    fn libraries(self, libraries: &[&str]) -> Self;
    fn emit(self, emit: &str) -> Self;
    fn as_args(&self) -> Vec<&str>;
}

impl ConfigBuilder for Config {
    fn input(mut self, input: &str) -> Self {
        self.input = input.to_owned();
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name.replace(name.to_owned());
        self
    }

    fn type_(mut self, build_type: BuildType) -> Self {
        self.type_.replace(build_type);
        self
    }

    fn output(mut self, output: &str) -> Self {
        self.output.replace(output.to_owned());
        self
    }

    fn out_dir(mut self, out_dir: &str) -> Self {
        self.out_dir.replace(out_dir.to_owned());
        self
    }

    fn path(mut self, path: &str) -> Self {
        self.paths.push(path.to_owned());
        self
    }

    fn paths(mut self, paths: &[&str]) -> Self {
        for path in paths {
            self.paths.push(path.to_string())
        }
        self
    }

    fn target_triple(mut self, target_triple: &str) -> Self {
        self.target_triple.replace(target_triple.to_owned());
        self
    }

    fn library(mut self, library: &str) -> Self {
        self.libraries.push(library.to_owned());
        self
    }

    fn libraries(mut self, libraries: &[&str]) -> Self {
        for library in libraries {
            self.libraries.push(library.to_string())
        }
        self
    }

    fn emit(mut self, emit: &str) -> Self {
        self.emit.replace(emit.to_owned());
        self
    }

    fn as_args(&self) -> Vec<&str> {
        let mut args: Vec<&str> = vec![&self.input];
        if let Some(out_dir) = &self.out_dir {
            args.extend(["--out-dir", out_dir]);
        }
        if let Some(name) = &self.name {
            args.extend(["--name", name]);
        }
        if let Some(type_) = self.type_ {
            args.extend(["--type", type_.as_str()]);
        }
        for library in self.libraries.iter() {
            args.extend(["--library", library]);
        }
        if let Some(target_triple) = &self.target_triple {
            args.extend(["--target-triple", target_triple]);
        }
        args
    }
}

impl<'ctx> From<&'ctx ArgMatches> for Config {
    fn from(matches: &'ctx ArgMatches) -> Self {
        Self {
            input: matches.get_one::<&str>("input").unwrap().to_string(),
            name: matches.get_one::<&str>("name").map(ToString::to_string),
            type_: matches.get_one::<&str>("type").map(|i|BuildType::from(*i)),
            output: matches.get_one::<&str>("output").map(ToString::to_string),
            out_dir: matches.get_one::<&str>("out-dir").map(ToString::to_string),
            paths: matches
                .get_many::<&str>("path")
                .unwrap()
                .map(ToString::to_string)
                .collect(),
            l: None,
            target_triple: matches.get_one::<&str>("target-triple").map(ToString::to_string),
            libraries: matches
                .get_many::<&str>("library")
                .unwrap()
                .map(ToString::to_string)
                .collect(),
            emit: matches.get_one::<&str>("emit").map(ToString::to_string),
        }
    }
}
