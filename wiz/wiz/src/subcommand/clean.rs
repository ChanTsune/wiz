use std::fs::remove_dir_all;
use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
use crate::core::{Cmd, Result};

pub(crate) struct CleanCommand;

impl Cmd for CleanCommand {
    const NAME: &'static str = "clean";

    fn command() -> Command {
        Command::new(Self::NAME)
            .about("Remove artifacts that wiz has generated in the past")
            .arg(Arg::new("target-dir").long("target-dir").help("Directory for all generated artifacts"))
    }

    fn execute(args: &ArgMatches) -> Result<()> {
        let options = Options::from(args);
        let target_dir = options.target_dir.unwrap_or_else(||PathBuf::from("target"));
        if target_dir.exists() {
            remove_dir_all(&target_dir)?;
        }
        Ok(())
    }
}

pub(crate) struct Options {
    target_dir: Option<PathBuf>
}

impl From<&ArgMatches> for Options {
    fn from(args: &ArgMatches) -> Self {
        Self {
            target_dir: args.get_one::<String>("target-dir").map(PathBuf::from)
        }
    }
}
