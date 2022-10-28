mod config;

use clap::builder::PossibleValuesParser;
use clap::{Arg, ArgAction, Command};
pub use config::{BuildType, Config, ConfigBuilder, ConfigExt, Emit, MessageFormat};

pub fn app(name: &'static str) -> Command {
    Command::new(name)
        .arg(position("input").required(true))
        .arg(long("name").num_args(1))
        .arg(
            long("type")
                .num_args(1)
                .value_parser(PossibleValuesParser::new(BuildType::all_str())),
        )
        .arg(short("output", 'o').num_args(1))
        .arg(long("out-dir").num_args(1))
        .arg(long("target-triple").num_args(1))
        .arg(short("path", 'p').action(ArgAction::Append).num_args(0..))
        .arg(short("L", 'L').action(ArgAction::Append).num_args(0..))
        .arg(long("library").action(ArgAction::Append).num_args(0..))
        .arg(
            long("emit")
                .num_args(1)
                .value_parser(PossibleValuesParser::new(Emit::all_str())),
        )
        .arg(long("message-format").num_args(1).value_parser(["json"]))
}

fn position(name: &'static str) -> Arg {
    Arg::new(name)
}

fn long(name: &'static str) -> Arg {
    Arg::new(name).long(name)
}

fn short(name: &'static str, s: char) -> Arg {
    Arg::new(name).short(s)
}

#[cfg(test)]
mod tests {
    use super::ConfigExt;
    use std::path::PathBuf;

    #[test]
    fn test_parse_arg() {
        let app = super::app("test");
        let matches = app.get_matches_from(&[
            "test",
            "main.wiz",
            "--library",
            "./std",
            "--library",
            "./libc",
        ]);
        let config = super::Config::from(&matches);

        assert_eq!(config.input().to_path_buf(), PathBuf::from("main.wiz"));
        assert_eq!(
            config.libraries(),
            vec![PathBuf::from("./std"), PathBuf::from("./libc")]
        )
    }
}
