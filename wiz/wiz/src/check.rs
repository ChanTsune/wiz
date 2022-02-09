use crate::core::workspace::construct_workspace_from;
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub(crate) fn check_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = options.value_of("manifest-path");
    let manifest_path = if let Some(manifest_path) = manifest_path {
        PathBuf::from(manifest_path).parent().unwrap().to_path_buf()
    } else {
        env::current_dir()?
    };
    let ws = construct_workspace_from(&manifest_path)?;
    if options.is_present("manifest") {
        println!("{:?}", ws.get_manifest()?);
    };
    Ok(())
}
