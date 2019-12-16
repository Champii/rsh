use clap::{App, Arg};
use std::process::Command;

use super::config::Config;

pub fn parse_config() -> Config {
    let version = format!("{} ({})", env!("VERGEN_SEMVER"), env!("VERGEN_SHA_SHORT"));

    let matches = App::new("RSH")
        .version(&*version)
        .about("Rust shell")
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

    Config { script_path }
}
