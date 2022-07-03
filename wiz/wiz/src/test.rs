use crate::core::dep::resolve_manifest_dependencies;
use crate::core::load_project;
use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "test";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let manifest_path = options.value_of("manifest-path");

    let another_std = options.value_of("std");

    let ws = load_project(manifest_path)?;
    println!("{:?}", ws);
    let resolved_dependencies =
        resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?, another_std)?;
    println!("{:?}", resolved_dependencies);
    todo!();
    Ok(())
}
