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

pub trait ConfigBuilder<'c> {
}

impl<'ctx> ConfigBuilder<'ctx> for Config<'ctx> {
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
