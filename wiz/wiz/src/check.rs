use crate::core::dep::resolve_manifest_dependencies;
use crate::core::load_project;
use crate::core::workspace::construct_workspace_from;
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub(crate) const COMMAND_NAME: &str = "check";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = options.value_of("manifest-path");

    let ws = load_project(manifest_path)?;

    if options.is_present("manifest") {
        println!("{:?}", ws.get_manifest()?);
    };
    let resolved_dependencies = resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?)?;
    println!("{:?}", resolved_dependencies);
    Ok(())
}
