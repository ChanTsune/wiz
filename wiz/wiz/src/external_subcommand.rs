use std::error::Error;
use clap::ArgMatches;

fn try_execute(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("external command {} {:?}", cmd, options);
    Ok(())
}