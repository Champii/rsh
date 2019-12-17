use clap::{App, Arg};

use super::config::Config;

pub fn parse_config() -> Config {
    let version = format!("{} ({})", env!("VERGEN_SEMVER"), env!("VERGEN_SHA_SHORT"));

    let matches = App::new("RSH")
        .version(&*version)
        .about("Rust shell")
        .arg(
            Arg::with_name("exec")
                .takes_value(true)
                .short("e")
                .help("Execute a piece of string")
                .required(false),
        )
        .arg(
            Arg::with_name("script")
                .help("Script to run")
                .index(1)
                .required(false),
        )
        .get_matches();

    let script_path = matches
        .value_of("script")
        .map(std::string::ToString::to_string);

    let input = matches
        .value_of("exec")
        .map(std::string::ToString::to_string);

    Config { script_path, input }
}
