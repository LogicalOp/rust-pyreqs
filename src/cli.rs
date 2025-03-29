use clap::{Command, Arg};
use std::path::PathBuf;

/// Struct to hold command-line arguments
pub struct CliArgs {
    pub directory: PathBuf,
    pub force: bool,
}

pub fn get_cli_args() -> CliArgs {
    let matches = Command::new("PyReqs")
        .version("1.0")
        .about("Generate requirements.txt for Python projects")
        .arg(
            Arg::new("directory")
                .help("The directory to scan for Python imports")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .value_name("DIRECTORY")
                .default_value("."),
        )
        .arg(
            Arg::new("force")
                .help("Force overwrite of requirements.txt")
                .long("force")
                .short('f')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let directory = matches
        .get_one::<String>("directory")
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    // Check if `--force` flag is set
    let force = matches.contains_id("force");

    CliArgs { directory, force }
}