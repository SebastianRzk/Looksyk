use crate::state::application_state::GraphRootLocation;

pub const DEFAULT_APPLICATION_PORT: u16 = 11000;
pub const APPLICATION_INTERNAL_HOST: &str = "127.0.0.1";
pub const APPLICATION_EXTERNAL_HOST: &str = "0.0.0.0";
pub const MAX_INLINE_FILESIZE: u64 = 1024 * 1024 * 16;
pub const DEFAULT_STATIC_PATH: &str = "./static";

#[derive(Debug)]
pub struct Configuration {
    pub application_port: u16,
    pub application_host: &'static str,
    pub max_inline_filesize: u64,
    pub static_path: String,
    pub overwrite_graph_location: Option<GraphRootLocation>,
}

pub fn get_default_configuration() -> Configuration {
    Configuration {
        application_port: DEFAULT_APPLICATION_PORT,
        application_host: APPLICATION_INTERNAL_HOST,
        max_inline_filesize: MAX_INLINE_FILESIZE,
        static_path: DEFAULT_STATIC_PATH.to_string(),
        overwrite_graph_location: None,
    }
}

impl Configuration {
    pub fn overwrite(&self, cli_args: CliArgs) -> Self {
        Configuration {
            application_host: application_host_from_cli_args(&cli_args),
            application_port: cli_args.port.unwrap_or(self.application_port),
            overwrite_graph_location: cli_args
                .graph_location
                .map(|s| GraphRootLocation { path: s.into() }),
            static_path: cli_args
                .static_path
                .unwrap_or_else(|| self.static_path.clone()),
            max_inline_filesize: self.max_inline_filesize,
        }
    }
}

#[derive(Debug)]
pub struct CliArgs {
    pub graph_location: Option<String>,
    pub port: Option<u16>,
    pub static_path: Option<String>,
    pub external_app: Option<bool>,
}

fn application_host_from_cli_args(cli_args: &CliArgs) -> &'static str {
    let is_external = cli_args.external_app.unwrap_or(false);
    if is_external {
        APPLICATION_EXTERNAL_HOST
    } else {
        APPLICATION_INTERNAL_HOST
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_overwrite_defaults_should_set_application_host_internal_when_nothing_set() {
        let default_config = super::get_default_configuration();
        let cli_args = super::CliArgs {
            graph_location: None,
            port: None,
            static_path: None,
            external_app: None,
        };

        let result = default_config.overwrite(cli_args);

        assert_eq!(result.application_host, super::APPLICATION_INTERNAL_HOST);
    }

    #[test]
    fn test_overwrite_defaults_should_set_application_host_external_when_external_app_set() {
        let default_config = super::get_default_configuration();
        let cli_args = super::CliArgs {
            graph_location: None,
            port: None,
            static_path: None,
            external_app: Some(true),
        };

        let result = default_config.overwrite(cli_args);

        assert_eq!(result.application_host, super::APPLICATION_EXTERNAL_HOST);
    }

    #[test]
    fn test_overwrite_defaults_should_set_application_host_internal_when_external_app_set_to_false()
    {
        let default_config = super::get_default_configuration();
        let cli_args = super::CliArgs {
            graph_location: None,
            port: None,
            static_path: None,
            external_app: Some(false),
        };

        let result = default_config.overwrite(cli_args);

        assert_eq!(result.application_host, super::APPLICATION_INTERNAL_HOST);
    }
}
