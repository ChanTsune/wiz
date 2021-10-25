use ansi_term::Color;
use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;

pub(crate) fn new_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut current = current_dir()?;
    let project_dir = options.value_of("project_name").unwrap();
    current.push(project_dir);
    println!(
        "{} project at {}",
        Color::Green.bold().paint("Created"),
        current.display()
    );
    Ok(())
}
