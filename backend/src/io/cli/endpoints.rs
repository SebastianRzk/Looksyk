use crate::configuration::CliArgs;
use crate::io::cli::dtos::CliArgsDto;
use crate::io::cli::mapper::map_to_io_cli;
use clap::Parser;

pub fn get_cli_args() -> CliArgs {
    map_to_io_cli(CliArgsDto::parse())
}
