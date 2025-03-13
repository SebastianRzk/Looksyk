use std::path::PathBuf;

use crate::io::fs::basic_file::read_file;
use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::data::config::runtime_graph_configuration::Config;
use crate::state::application_state::GraphRootLocation;

pub fn read_config_from_file(data_path: &GraphRootLocation) -> Config {
    let config_file_content_as_str = read_file(config_path(data_path));
    serde_json::from_str(config_file_content_as_str.as_str()).unwrap()
}

pub fn save_config_to_file(data_path: &GraphRootLocation, config: &Config) {
    let config_file_content_as_str = serde_json::to_string_pretty(config).unwrap();
    std::fs::write(config_path(data_path), config_file_content_as_str).unwrap();
}

fn config_path(data_path: &GraphRootLocation) -> PathBuf {
    data_path.path.clone().join(REL_CONFIG_PATH)
}
