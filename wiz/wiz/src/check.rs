use crate::core::dep::resolve_manifest_dependencies;
use crate::core::load_project;
use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "check";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let manifest_path = options.get_one::<String>("manifest-path").map(|i|i.as_str());

    let another_std = options.get_one::<String>("std").map(|i|i.as_str());

    let ws = load_project(manifest_path)?;

    if options.get_flag("manifest") {
        println!("{:?}", ws.get_manifest()?);
    };
    let resolved_dependencies =
        resolve_manifest_dependencies(&ws.cws, &ws.get_manifest()?, another_std)?;
    println!("{:?}", resolved_dependencies);
    Ok(())
}
