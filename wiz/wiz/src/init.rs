use std::env::current_dir;
use std::error::Error;
use ansi_term::Color;
use clap::ArgMatches;

pub(crate) fn init_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let current = current_dir()?;
    println!("{} project at {}", Color::Green.bold().paint("Created"), current.display());
    Ok(())
}
