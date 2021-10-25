use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;

pub(crate) fn new_command(name: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let current = current_dir()?;
    println!("{}", current.display());
    Ok(())
}
