use crate::core::create_project;
use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;
use std::fs::create_dir_all;

pub(crate) const COMMAND_NAME: &str = "new";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut current = current_dir()?;
    let project_dir = options.value_of("path").unwrap();
    current.push(project_dir);
    create_dir_all(&current)?;
    create_project(&current, project_dir)?;
    if !options.is_present("quite") {
        println!(
            "{} project at {}",
            Color::Green.bold().paint("Created"),
            current.display()
        );
    };
    Ok(())
}
