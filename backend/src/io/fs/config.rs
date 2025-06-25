use crate::io::fs::basic_file::read_file;
use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::data::config::runtime_graph_configuration::{
    Appearance, Config, Design, Favourite,
};
use crate::state::application_state::GraphRootLocation;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOnDisk {
    pub favourites: Vec<Favourite>,
    pub design: DesignOnDisk,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DesignOnDisk {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
    pub appearance: String,
}

pub fn read_config_from_file(data_path: &GraphRootLocation) -> Config {
    let config_file_content_as_str = read_file(config_path(data_path));
    serde_json::from_str::<ConfigOnDisk>(config_file_content_as_str.as_str())
        .map(convert_config_on_disk_to_config)
        .expect("Failed to read config from file")
}

fn convert_config_on_disk_to_config(config_on_disk: ConfigOnDisk) -> Config {
    Config {
        favourites: config_on_disk.favourites,
        design: Design {
            primary_color: config_on_disk.design.primary_color,
            background_color: config_on_disk.design.background_color,
            foreground_color: config_on_disk.design.foreground_color,
            primary_shading: config_on_disk.design.primary_shading,
            appearance: Appearance::from_str(&config_on_disk.design.appearance)
                .expect("Failed to parse appearance from config"),
        },
        title: config_on_disk.title,
    }
}

fn convert_config_to_on_disk(config: &Config) -> ConfigOnDisk {
    ConfigOnDisk {
        favourites: config.favourites.clone(),
        design: DesignOnDisk {
            primary_color: config.design.primary_color.clone(),
            background_color: config.design.background_color.clone(),
            foreground_color: config.design.foreground_color.clone(),
            primary_shading: config.design.primary_shading.clone(),
            appearance: config.design.appearance.to_string(),
        },
        title: config.title.clone(),
    }
}

pub fn save_config_to_file(data_path: &GraphRootLocation, config: &Config) {
    let config_file_content_as_str =
        serde_json::to_string_pretty(&convert_config_to_on_disk(config))
            .expect("Failed to serialize config to JSON");
    std::fs::write(config_path(data_path), config_file_content_as_str).unwrap();
}

fn config_path(data_path: &GraphRootLocation) -> PathBuf {
    data_path.path.clone().join(REL_CONFIG_PATH)
}
