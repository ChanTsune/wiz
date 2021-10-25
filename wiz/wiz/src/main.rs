mod new;

use crate::new::new_command;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz").subcommand(
        SubCommand::with_name("new").arg(Arg::with_name("project_name").required(true)),
    );
    let matches = app.get_matches();
    match matches.subcommand() {
        ("new", Some(option)) => {
            new_command("new", option)?;
        }
        _ => {}
    }
    Ok(())
}
