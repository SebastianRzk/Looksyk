use std::clone::Clone;
use std::env::home_dir;
use std::path::Path;
use std::string::ToString;

use serde::{Deserialize, Serialize};

use crate::io::fs::basic_file::{create_folder, exists_file, exists_folder, read_file};
use crate::state::state::DataRootLocation;

const CONFIG_FILE_NAME: &str = "config.json";
const INITIAL_GRAPH_NAME: &str = "MyFirstGraph";
const INITIAL_GRAPH_PATH: &str = "/graph";


pub fn get_current_active_data_root_location(config_location: &InitialConfigLocation) -> DataRootLocation {
    let path = Path::new(config_location.path.as_str()).to_path_buf();
    if !exists_folder(path.clone()) {
        create_folder(path.clone());
    }

    let file = path.clone().join(CONFIG_FILE_NAME);

    if !exists_file(file.clone()) {
        let config_file_content_as_str = serde_json::to_string(&RootPathConfig {
            current_active: RootPath {
                name: INITIAL_GRAPH_NAME.to_string(),
                path: format!("{}/{}", home_dir().unwrap().as_os_str().to_str().unwrap(), INITIAL_GRAPH_PATH)
            },
            available: vec![],
        }).unwrap();
        std::fs::write(file.clone(), config_file_content_as_str).unwrap();
    }

    let config_content = read_file(file);
    let root_path_config: RootPathConfig = serde_json::from_str(&config_content).unwrap();
    DataRootLocation {
        path: root_path_config.current_active.path.to_string()
    }
}


pub struct InitialConfigLocation {
    pub path: String,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootPathConfig {
    pub current_active: RootPath,
    pub available: Vec<RootPath>,
}


#[derive(Serialize, Deserialize)]
pub struct RootPath {
    pub path: String,
    pub name: String,
}