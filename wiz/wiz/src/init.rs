use crate::core::create_project;
use crate::core::error::CliError;
use crate::core::Result;
use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;

pub(crate) const COMMAND_NAME: &str = "init";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let current = current_dir()?;
    let project_name = current.iter().last();
    if !options.is_present("overwrite") && current.read_dir()?.next().is_some() {
        return Err(Box::new(CliError::from(format!(
            "`{}` is not empty",
            current.display()
        ))));
    };
    create_project(&current, project_name.unwrap().to_str().unwrap())?;
    if !options.is_present("quite") {
        println!(
            "{} project at {}",
            Color::Green.bold().paint("Created"),
            current.display()
        );
    };
    Ok(())
}
