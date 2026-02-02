use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::data::config::runtime_graph_configuration::{
    Appearance, JournalTitleFormat, ShowWeekdayInTitle,
};
use crate::migration::migration_1_10_2::{ConfigOnDiskV1_10_2, FavouriteV1_10_0};
use crate::state::application_state::GraphRootLocation;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOnDiskV1_18_0 {
    pub favourites: Vec<FavouriteV1_10_0>,
    pub design: DesignOnDiskV1_18_0,
    pub title: Option<String>,
    pub journal_configuration: JournalConfigrationOnDiskV1_18_0,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DesignOnDiskV1_18_0 {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
    pub appearance: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JournalConfigrationOnDiskV1_18_0 {
    pub journal_title_format: String,
    pub show_weekday_in_title: String,
}

pub fn migriere_1_18_0(user_application_directory: &GraphRootLocation) {
    let config_location = user_application_directory.path.join(REL_CONFIG_PATH);
    if config_location.exists() {
        println!("Found config file, migrating...");

        // Read the old config file
        let old_config_content =
            fs::read_to_string(&config_location).expect("Failed to read old config file");
        let old_config: ConfigOnDiskV1_10_2 =
            serde_json::from_str(&old_config_content).expect("Failed to parse old config file");

        // Convert to new format
        let new_config = ConfigOnDiskV1_18_0 {
            favourites: old_config.favourites,
            design: DesignOnDiskV1_18_0 {
                primary_color: old_config.design.primary_color,
                background_color: old_config.design.background_color,
                foreground_color: old_config.design.foreground_color,
                primary_shading: old_config.design.primary_shading,
                appearance: Appearance::Dark.to_string(),
            },
            title: old_config.title,
            journal_configuration: JournalConfigrationOnDiskV1_18_0 {
                journal_title_format: JournalTitleFormat::World.to_string(),
                show_weekday_in_title: ShowWeekdayInTitle::None.to_string(),
            },
        };

        // Write the new config file
        save_config_to_file(user_application_directory, &new_config);
        println!("Migration completed successfully.");
    } else {
        println!("No shares file found, no migration needed.");
    }
}

fn save_config_to_file(data_path: &GraphRootLocation, config: &ConfigOnDiskV1_18_0) {
    let config_file_content_as_str =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    fs::write(
        crate::io::fs::config::config_path(data_path),
        config_file_content_as_str,
    )
    .unwrap();
}
