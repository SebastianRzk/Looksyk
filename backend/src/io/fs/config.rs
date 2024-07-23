use std::path::{Path, PathBuf};

use crate::io::fs::basic_file::read_file;
use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::config::config::Config;
use crate::state::state::DataRootLocation;

pub fn read_config_from_file(data_path: &DataRootLocation) -> Config {
    let config_file_content_as_str = read_file(config_path(data_path));
    let json: Config = serde_json::from_str(config_file_content_as_str.as_str()).unwrap();
    return json;
}

pub fn save_config_to_file(data_path: &DataRootLocation, config: &Config) {
    let config_file_content_as_str = serde_json::to_string(config).unwrap();
    std::fs::write(config_path(data_path), config_file_content_as_str).unwrap();
}

fn config_path(data_path: &DataRootLocation) -> PathBuf {
    Path::new(data_path.path.as_str()).join(REL_CONFIG_PATH)
}