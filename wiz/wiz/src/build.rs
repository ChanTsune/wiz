use std::error::Error;
use clap::ArgMatches;

pub(crate) fn build_command(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}
