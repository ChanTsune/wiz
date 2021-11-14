use std::path::{Path, PathBuf};
use clap::ArgMatches;

pub struct Config<'ctx> {
    input: &'ctx str,
    name: Option<&'ctx str>,
    type_: Option<&'ctx str>,
    output: Option<&'ctx str>,
    out_dir: Option<&'ctx str>,
    paths: Vec<String>,
    l: Option<&'ctx str>,
}

impl<'ctx> Config<'ctx> {
    pub(crate) fn input(&self) -> &Path {
        Path::new(self.input)
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
}

impl<'ctx> From<&'ctx ArgMatches<'ctx>> for Config<'ctx>{
    fn from(matches: &'ctx ArgMatches<'ctx>) -> Self {
        Self {
            input: matches.value_of("input").unwrap(),
            name: matches.value_of("name"),
            type_: matches.value_of("type"),
            output: matches.value_of("output"),
            out_dir: matches.value_of("out-dir"),
            paths: matches.values_of_lossy("path").unwrap_or_default(),
            l: None
        }
    }
}
