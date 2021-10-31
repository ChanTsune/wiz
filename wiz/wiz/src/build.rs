use clap::ArgMatches;
use std::error::Error;

pub(crate) fn build_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    super::subcommand::execute("wizc", &[])
}
