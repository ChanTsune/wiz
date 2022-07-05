mod config;

use clap::{Arg, Command};
pub use config::build_type::BuildType;
pub use config::{Config, ConfigBuilder, ConfigExt};

pub fn app(name: &str) -> Command {
    Command::new(name)
        .arg(position("input").required(true))
        .arg(long("name").takes_value(true))
        .arg(
            long("type")
                .takes_value(true)
                .possible_values(BuildType::all_str()),
        )
        .arg(short("output", 'o').takes_value(true))
        .arg(long("out-dir").takes_value(true))
        .arg(long("target-triple").takes_value(true))
        .arg(
            short("path", 'p')
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(short("L", 'L').takes_value(true).multiple_occurrences(true))
        .arg(long("library").takes_value(true).multiple_occurrences(true))
        .arg(
            long("emit")
                .takes_value(true)
                .possible_values(&["llvm-ir", "object", "asm"]),
        )
}

fn position(name: &str) -> Arg {
    Arg::new(name)
}

fn long(name: &str) -> Arg {
    Arg::new(name).long(name)
}

fn short(name: &str, s: char) -> Arg {
    Arg::new(name).short(s)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::ConfigExt;

    #[test]
    fn test_parse_arg() {
        let app = super::app("test");
        let matches = app.get_matches_from(&["test", "main.wiz"]);
        let config = super::Config::from(&matches);
        assert_eq!(config.input().to_path_buf(), PathBuf::from("main.wiz"));
    }
}
