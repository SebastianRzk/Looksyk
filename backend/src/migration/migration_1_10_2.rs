use crate::io::fs::config::DesignOnDisk;
use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::data::config::runtime_graph_configuration::{Appearance, Favourite};
use crate::state::application_state::GraphRootLocation;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Clone)]
pub struct OldConfigOnDisk {
    pub favourites: Vec<Favourite>,
    pub design: OldDesignOnDisk,
    pub title: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OldDesignOnDisk {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOnDiskV1_10_2 {
    pub favourites: Vec<Favourite>,
    pub design: DesignOnDisk,
    pub title: Option<String>,
}

pub fn migriere_1_10_2(user_application_directory: &GraphRootLocation) {
    let config_location = user_application_directory.path.join(REL_CONFIG_PATH);
    if config_location.exists() {
        println!("Found config file, migrating...");

        // Read the old config file
        let old_config_content =
            fs::read_to_string(&config_location).expect("Failed to read old config file");
        let old_config: OldConfigOnDisk =
            serde_json::from_str(&old_config_content).expect("Failed to parse old config file");

        // Convert to new format
        let new_config = ConfigOnDiskV1_10_2 {
            favourites: old_config.favourites,
            design: DesignOnDisk {
                primary_color: old_config.design.primary_color,
                background_color: old_config.design.background_color,
                foreground_color: old_config.design.foreground_color,
                primary_shading: old_config.design.primary_shading,
                appearance: Appearance::Dark.to_string(),
            },
            title: old_config.title,
        };

        // Write the new config file
        save_config_to_file(user_application_directory, &new_config);
        println!("Migration completed successfully.");
    } else {
        println!("No shares file found, no migration needed.");
    }
}

fn save_config_to_file(data_path: &GraphRootLocation, config: &ConfigOnDiskV1_10_2) {
    let config_file_content_as_str =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    fs::write(
        crate::io::fs::config::config_path(data_path),
        config_file_content_as_str,
    )
    .unwrap();
}
