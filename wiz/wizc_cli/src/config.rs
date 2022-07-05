pub(crate) mod build_type;

use crate::config::build_type::BuildType;
use clap::ArgMatches;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Config<'ctx> {
    input: &'ctx str,
    name: Option<&'ctx str>,
    type_: Option<BuildType>,
    output: Option<&'ctx str>,
    out_dir: Option<&'ctx str>,
    paths: Vec<&'ctx str>,
    l: Option<&'ctx str>,
    target_triple: Option<&'ctx str>,
    libraries: Vec<&'ctx str>,
    emit: Option<&'ctx str>,
}

pub trait ConfigExt<'ctx> {
    fn input(&self) -> &Path;
    fn name(&self) -> Option<&'ctx str>;
    fn type_(&self) -> BuildType;
    fn output(&self) -> Option<&'ctx str>;
    fn out_dir(&self) -> Option<&'ctx str>;
    fn paths(&self) -> Vec<PathBuf>;
    fn target_triple(&self) -> Option<&'ctx str>;
    fn libraries(&self) -> Vec<PathBuf>;
    fn emit(&self) -> Option<&'ctx str>;
}

impl<'ctx> ConfigExt<'ctx> for Config<'ctx> {
    fn input(&self) -> &Path {
        Path::new(self.input)
    }

    fn name(&self) -> Option<&'ctx str> {
        self.name
    }

    fn type_(&self) -> BuildType {
        self.type_.unwrap_or(BuildType::Binary)
    }

    fn output(&self) -> Option<&'ctx str> {
        self.output
    }

    fn out_dir(&self) -> Option<&'ctx str> {
        self.out_dir
    }

    fn paths(&self) -> Vec<PathBuf> {
        self.paths.iter().map(PathBuf::from).collect()
    }

    fn target_triple(&self) -> Option<&'ctx str> {
        self.target_triple
    }

    fn libraries(&self) -> Vec<PathBuf> {
        self.libraries.iter().map(PathBuf::from).collect()
    }

    fn emit(&self) -> Option<&'ctx str> {
        self.emit
    }
}

pub trait ConfigBuilder<'ctx> {
    fn input(self, input: &'ctx str) -> Self;
    fn name(self, name: &'ctx str) -> Self;
    fn type_(self, build_type: BuildType) -> Self;
    fn output(self, output: &'ctx str) -> Self;
    fn out_dir(self, out_dir: &'ctx str) -> Self;
    fn path(self, path: &'ctx str) -> Self;
    fn paths(self, paths: &[&'ctx str]) -> Self;
    fn target_triple(self, target_triple: &'ctx str) -> Self;
    fn library(self, library: &'ctx str) -> Self;
    fn libraries(self, libraries: &[&'ctx str]) -> Self;
    fn emit(self, emit: &'ctx str) -> Self;
    fn as_args(&self) -> Vec<&'ctx str>;
}

impl<'ctx> ConfigBuilder<'ctx> for Config<'ctx> {
    fn input(mut self, input: &'ctx str) -> Self {
        self.input = input;
        self
    }

    fn name(mut self, name: &'ctx str) -> Self {
        self.name.replace(name);
        self
    }

    fn type_(mut self, build_type: BuildType) -> Self {
        self.type_.replace(build_type);
        self
    }

    fn output(mut self, output: &'ctx str) -> Self {
        self.output.replace(output);
        self
    }

    fn out_dir(mut self, out_dir: &'ctx str) -> Self {
        self.out_dir.replace(out_dir);
        self
    }

    fn path(mut self, path: &'ctx str) -> Self {
        self.paths.push(path);
        self
    }

    fn paths(mut self, path: &[&'ctx str]) -> Self {
        self.paths.extend(path);
        self
    }

    fn target_triple(mut self, target_triple: &'ctx str) -> Self {
        self.target_triple.replace(target_triple);
        self
    }

    fn library(mut self, library: &'ctx str) -> Self {
        self.libraries.push(library);
        self
    }

    fn libraries(mut self, libraries: &[&'ctx str]) -> Self {
        self.libraries.extend(libraries);
        self
    }

    fn emit(mut self, emit: &'ctx str) -> Self {
        self.emit.replace(emit);
        self
    }

    fn as_args(&self) -> Vec<&'ctx str> {
        let mut args = vec![self.input];
        if let Some(out_dir) = self.out_dir {
            args.extend(["--out-dir", out_dir]);
        }
        if let Some(name) = self.name {
            args.extend(["--name", name]);
        }
        if let Some(type_) = self.type_ {
            args.extend(["--type", type_.as_str()]);
        }
        for library in self.libraries.iter() {
            args.extend(["--library", library]);
        }
        if let Some(target_triple) = self.target_triple {
            args.extend(["--target-triple", target_triple]);
        }
        args
    }
}

impl<'ctx> From<&'ctx ArgMatches> for Config<'ctx> {
    fn from(matches: &'ctx ArgMatches) -> Self {
        Self {
            input: matches.value_of("input").unwrap(),
            name: matches.value_of("name"),
            type_: matches.value_of("type").map(BuildType::from),
            output: matches.value_of("output"),
            out_dir: matches.value_of("out-dir"),
            paths: matches.values_of("path").unwrap_or_default().collect(),
            l: None,
            target_triple: matches.value_of("target-triple"),
            libraries: matches.values_of("library").unwrap_or_default().collect(),
            emit: matches.value_of("emit"),
        }
    }
}
