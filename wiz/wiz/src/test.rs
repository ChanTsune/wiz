use crate::core::load_project;
use clap::ArgMatches;
use std::error::Error;

pub(crate) const COMMAND_NAME: &str = "test";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = options.value_of("manifest-path");
    let ws = load_project(manifest_path)?;
    println!("{:?}", ws);
    todo!();
    Ok(())
}
