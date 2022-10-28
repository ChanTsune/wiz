mod build_type;
mod message_format;

pub use build_type::BuildType;
use clap::ArgMatches;
pub use message_format::MessageFormat;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct Config {
    input: String,
    name: Option<String>,
    type_: Option<BuildType>,
    output: Option<String>,
    out_dir: Option<PathBuf>,
    paths: Vec<PathBuf>,
    l: Option<String>,
    target_triple: Option<String>,
    libraries: Vec<PathBuf>,
    emit: Option<String>,
    message_format: Option<MessageFormat>,
}

pub trait ConfigExt {
    fn input(&self) -> &Path;
    fn name(&self) -> Option<&str>;
    fn type_(&self) -> BuildType;
    fn output(&self) -> Option<String>;
    fn out_dir(&self) -> Option<PathBuf>;
    fn paths(&self) -> Vec<PathBuf>;
    fn target_triple(&self) -> Option<String>;
    fn libraries(&self) -> Vec<PathBuf>;
    fn emit(&self) -> Option<String>;
    fn message_format(&self) -> MessageFormat;
}

impl ConfigExt for Config {
    fn input(&self) -> &Path {
        Path::new(&self.input)
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn type_(&self) -> BuildType {
        self.type_.unwrap_or(BuildType::Binary)
    }

    fn output(&self) -> Option<String> {
        self.output.clone()
    }

    fn out_dir(&self) -> Option<PathBuf> {
        self.out_dir.clone()
    }

    fn paths(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }

    fn target_triple(&self) -> Option<String> {
        self.target_triple.clone()
    }

    fn libraries(&self) -> Vec<PathBuf> {
        self.libraries.clone()
    }

    fn emit(&self) -> Option<String> {
        self.emit.clone()
    }

    fn message_format(&self) -> MessageFormat {
        self.message_format.unwrap_or_default()
    }
}

pub trait ConfigBuilder {
    fn input(self, input: &str) -> Self;
    fn name(self, name: &str) -> Self;
    fn type_(self, build_type: BuildType) -> Self;
    fn output(self, output: &str) -> Self;
    fn out_dir<P: AsRef<Path>>(self, out_dir: P) -> Self;
    fn path<P: AsRef<Path>>(self, path: P) -> Self;
    fn paths<P: AsRef<Path>>(self, paths: &[P]) -> Self;
    fn target_triple(self, target_triple: &str) -> Self;
    fn library<P: AsRef<Path>>(self, library: P) -> Self;
    fn libraries<P: AsRef<Path>>(self, libraries: &[P]) -> Self;
    fn emit(self, emit: &str) -> Self;
    fn message_format(self, message_format: MessageFormat) -> Self;
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

    fn out_dir<P: AsRef<Path>>(mut self, out_dir: P) -> Self {
        self.out_dir.replace(out_dir.as_ref().to_owned());
        self
    }

    fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.paths.push(path.as_ref().to_owned());
        self
    }

    fn paths<P: AsRef<Path>>(mut self, paths: &[P]) -> Self {
        for path in paths {
            self.paths.push(path.as_ref().to_owned())
        }
        self
    }

    fn target_triple(mut self, target_triple: &str) -> Self {
        self.target_triple.replace(target_triple.to_owned());
        self
    }

    fn library<P: AsRef<Path>>(mut self, library: P) -> Self {
        self.libraries.push(library.as_ref().to_owned());
        self
    }

    fn libraries<P: AsRef<Path>>(mut self, libraries: &[P]) -> Self {
        for library in libraries {
            self.libraries.push(library.as_ref().to_owned())
        }
        self
    }

    fn emit(mut self, emit: &str) -> Self {
        self.emit.replace(emit.to_owned());
        self
    }

    fn message_format(mut self, message_format: MessageFormat) -> Self {
        self.message_format.replace(message_format);
        self
    }

    fn as_args(&self) -> Vec<&str> {
        let mut args: Vec<&str> = vec![&self.input];
        if let Some(out_dir) = &self.out_dir {
            args.extend(["--out-dir", out_dir.as_os_str().to_str().unwrap()]);
        }
        if let Some(name) = &self.name {
            args.extend(["--name", name]);
        }
        if let Some(type_) = self.type_ {
            args.extend(["--type", type_.as_str()]);
        }
        for library in self.libraries.iter() {
            args.extend(["--library", library.as_os_str().to_str().unwrap()]);
        }
        if let Some(target_triple) = &self.target_triple {
            args.extend(["--target-triple", target_triple]);
        }
        if let Some(message_format) = &self.message_format {
            args.extend(["--message-format", message_format.as_str()]);
        };
        args
    }
}

impl<'ctx> From<&'ctx ArgMatches> for Config {
    fn from(matches: &'ctx ArgMatches) -> Self {
        Self {
            input: matches
                .get_one::<String>("input")
                .expect("input is required")
                .to_string(),
            name: matches.get_one::<String>("name").map(ToString::to_string),
            type_: matches
                .get_one::<String>("type")
                .map(|i| BuildType::from(i.as_str())),
            output: matches.get_one::<String>("output").map(ToString::to_string),
            out_dir: matches.get_one::<String>("out-dir").map(PathBuf::from),
            paths: matches
                .get_many::<String>("path")
                .map(|i| i.map(PathBuf::from).collect())
                .unwrap_or_default(),
            l: None,
            target_triple: matches
                .get_one::<String>("target-triple")
                .map(ToString::to_string),
            libraries: matches
                .get_many::<String>("library")
                .map(|i| i.map(PathBuf::from).collect())
                .unwrap_or_default(),
            emit: matches.get_one::<String>("emit").map(ToString::to_string),
            message_format: matches
                .get_one::<String>("message-format")
                .map(|s| MessageFormat::from(s.as_str())),
        }
    }
}
