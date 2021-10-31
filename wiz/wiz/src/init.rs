use crate::core::create_project;
use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;

pub(crate) fn init_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let current = current_dir()?;
    let project_name = current.iter().last();
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
