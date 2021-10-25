use crate::common::create_project;
use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;
use std::fs::create_dir_all;

pub(crate) fn new_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut current = current_dir()?;
    let project_dir = options.value_of("project_name").unwrap();
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
