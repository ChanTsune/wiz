mod new;
mod init;

use crate::new::new_command;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;
use crate::init::init_command;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz").subcommand(
        SubCommand::with_name("new").arg(Arg::with_name("project_name").required(true)),
    ).subcommand(SubCommand::with_name("init"));
    let matches = app.get_matches();
    match matches.subcommand() {
        ("new", Some(option)) => {
            new_command("new", option)?;
        }
        ("init", Some(option)) => {
            init_command("init", option)?;
        }
        _ => {}
    }
    Ok(())
}
