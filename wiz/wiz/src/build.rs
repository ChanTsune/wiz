use clap::ArgMatches;
use std::error::Error;

pub(crate) fn build_command(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}
