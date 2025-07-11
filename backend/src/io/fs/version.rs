use crate::io::fs::paths::{REL_CONFIG_DIRECTORY, VERSION_FILE_NAME};
use crate::migration::model::ApplicationVersion;
use crate::state::application_state::GraphRootLocation;

const UNKNOWN_VERSION: &str = "0.0.0";

pub fn load_graph_version(graph_root_location: &GraphRootLocation) -> ApplicationVersion {
    let version_path = graph_root_location
        .path
        .join(REL_CONFIG_DIRECTORY)
        .join(VERSION_FILE_NAME);
    let version_string =
        std::fs::read_to_string(version_path).unwrap_or_else(|_| UNKNOWN_VERSION.to_string());
    ApplicationVersion::new(version_string.trim())
}

pub fn save_graph_version(graph_root_location: &GraphRootLocation, version: &ApplicationVersion) {
    let version_path = graph_root_location
        .path
        .join(REL_CONFIG_DIRECTORY)
        .join(VERSION_FILE_NAME);
    std::fs::write(version_path, version.to_string()).expect("Could not write version file");
}
