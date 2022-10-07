use crate::core::create_project;
use crate::core::Result;
use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;
use std::fs::create_dir_all;

pub(crate) const COMMAND_NAME: &str = "new";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let mut current = current_dir()?;
    let project_dir = options
        .get_one::<String>("path")
        .map(|i| i.as_str())
        .unwrap();
    current.push(project_dir);
    create_dir_all(&current)?;
    create_project(&current, project_dir)?;
    if !options.get_flag("quite") {
        println!(
            "{} project at {}",
            Color::Green.bold().paint("Created"),
            current.display()
        );
    };
    Ok(())
}
