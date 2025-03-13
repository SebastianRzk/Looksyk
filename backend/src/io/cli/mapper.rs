use crate::io::cli::dtos::CliArgsDto;
use crate::looksyk::data::config::startup_configuration::CliArgs;

pub fn map_to_io_cli(dto: CliArgsDto) -> CliArgs {
    CliArgs {
        graph_location: dto.graph_location,
        port: dto.port,
        static_path: dto.static_path,
        external_app: dto.external_app,
    }
}
