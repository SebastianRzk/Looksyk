use clap::Parser;

#[derive(Parser)]
pub struct CliArgsDto {
    #[arg(long, value_name = "graph-location")]
    pub graph_location: Option<String>,
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,
}

