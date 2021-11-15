use crate::core::error::CliError;
use crate::core::workspace::construct_workspace_from;
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fs::create_dir_all;
use std::option::Option::Some;
use std::path::PathBuf;

pub(crate) fn build_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let ws = construct_workspace_from(env::current_dir()?)?;

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
        let mut current_dir = env::current_dir()?;
        current_dir.push("target");
        current_dir
    };

    let mut args = vec![ws.cws.to_str().unwrap()];
    args.extend(["--out-dir", &target_dir.to_str().unwrap()]);

    args.extend(["--name", ws.cws.file_name().unwrap().to_str().unwrap()]);
    args.extend(["--type", "bin"]);

    if let Some(target_triple) = options.value_of("target-triple") {
        args.extend(["--target-triple", target_triple]);
    };

    create_dir_all(&target_dir)?;
    super::subcommand::execute("wizc", &args)
}
