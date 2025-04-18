use clap::Parser;

#[derive(Parser)]
pub struct CliArgsDto {
    #[arg(long, value_name = "graph-location")]
    pub graph_location: Option<String>,
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,
    #[arg(long, value_name = "static-path")]
    pub static_path: Option<String>,
    #[arg(long, value_name = "external-app")]
    pub external_app: Option<bool>,
}
