use clap::ArgMatches;
use std::path::{Path, PathBuf};

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
}

impl<'ctx> Config<'ctx> {
    pub(crate) fn input(&self) -> &Path {
        Path::new(self.input)
    }

    pub(crate) fn name(&self) -> Option<&'ctx str> {
        self.name
    }

    pub(crate) fn type_(&self) -> Option<BuildType> {
        self.type_
    }

    pub(crate) fn output(&self) -> Option<&'ctx str> {
        self.output
    }

    pub(crate) fn out_dir(&self) -> Option<&'ctx str> {
        self.out_dir
    }

    pub(crate) fn paths(&self) -> Vec<PathBuf> {
        self.paths.iter().map(PathBuf::from).collect()
    }

    pub(crate) fn target_triple(&self) -> Option<&'ctx str> {
        self.target_triple
    }

    pub(crate) fn libraries(&self) -> Vec<PathBuf> {
        self.libraries.iter().map(PathBuf::from).collect()
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildType {
    Binary,
    Library,
    Test,
}

impl BuildType {
    pub fn all_str() -> &'static [&'static str] {
        &["bin", "lib", "test"]
    }
}

impl From<&str> for BuildType {
    fn from(s: &str) -> Self {
        match s {
            "bin" => BuildType::Binary,
            "lib" => BuildType::Library,
            "test" => BuildType::Test,
            _ => panic!("Unknown build type: {}", s),
        }
    }
}
