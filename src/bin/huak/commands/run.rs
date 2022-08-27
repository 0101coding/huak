use clap::{arg, value_parser, Arg, ArgMatches};
use huak::errors::CliResult;

pub fn arg() -> Arg<'static> {
    arg!("run")
        .multiple_values(true)
        .value_parser(value_parser!(String))
        .help("Run a command within the project's environment context.")
}

pub fn run(_args: &ArgMatches) -> CliResult {
    unimplemented!()
}