use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use crate::error::CliError;

pub(crate) fn build_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let wizc = "wizc";
    let mut args = vec![wizc];
    let target_dir = if let Some(target_dir) = options.value_of("target-dir") {
        let d = PathBuf::from(target_dir);
        if d.exists() && !d.is_dir() {
            return Err(Box::from(CliError::from(format!("{} is not directory", d.display()))))
        } else {
            d
        }
    } else {
        let mut current_dir = env::current_dir()?;
        current_dir.push("target");
        current_dir
    };
    args.extend(["--target-dir", &target_dir.to_str().unwrap()]);
    create_dir_all(&target_dir)?;
    super::subcommand::execute(wizc, &args)
}
