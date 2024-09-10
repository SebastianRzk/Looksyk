use crate::state::state::DataRootLocation;

pub const DEFAULT_APPLICATION_PORT: u16 = 8989;
pub const APPLICATION_HOST: &str = "127.0.0.1";

pub const MAX_INLINE_FILESIZE: u64 = 1024 * 1024 * 16;

pub struct Configuration {
    pub application_port: u16,
    pub application_host: String,
    pub max_inline_filesize: u64,
    pub overwrite_graph_location: Option<DataRootLocation>,
}

pub fn get_default_configuration() -> Configuration {
    Configuration {
        application_port: DEFAULT_APPLICATION_PORT,
        application_host: APPLICATION_HOST.to_string(),
        max_inline_filesize: MAX_INLINE_FILESIZE,
        overwrite_graph_location: None,
    }
}


impl Configuration {
    pub fn overwrite(&self, cli_args: CliArgs) -> Self {
        Configuration {
            application_port: cli_args.port.unwrap_or(self.application_port),
            overwrite_graph_location: cli_args.graph_location.map(|s| DataRootLocation {
                path: s.into(),
                name: "default".to_string(),
            }),
            application_host: self.application_host.clone(),
            max_inline_filesize: self.max_inline_filesize,
        }
    }
}


pub struct CliArgs {
    pub graph_location: Option<String>,
    pub port: Option<u16>,
}
