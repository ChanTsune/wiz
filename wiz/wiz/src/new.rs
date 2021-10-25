use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;
use ansi_term::Color;

pub(crate) fn new_command(name: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let current = current_dir()?;
    println!("{} project at {1}", Color::Green.bold().paint("Created"), current.display());
    Ok(())
}
