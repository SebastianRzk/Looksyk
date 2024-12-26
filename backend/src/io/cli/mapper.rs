use crate::configuration::CliArgs;
use crate::io::cli::dtos::CliArgsDto;

pub fn map_to_io_cli(dto: CliArgsDto) -> CliArgs {
    CliArgs {
        graph_location: dto.graph_location,
        port: dto.port,
        application_title: dto.title,
    }
}
