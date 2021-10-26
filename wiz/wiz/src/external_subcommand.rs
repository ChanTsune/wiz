use clap::ArgMatches;
use std::error::Error;

fn try_execute(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("external command {} {:?}", cmd, options);
    Ok(())
}
