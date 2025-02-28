use crate::io::cli::dtos::CliArgsDto;
use crate::looksyk::config::startup_configuration::CliArgs;

pub fn map_to_io_cli(dto: CliArgsDto) -> CliArgs {
    CliArgs {
        graph_location: dto.graph_location,
        port: dto.port,
        application_title: dto.title,
    }
}
