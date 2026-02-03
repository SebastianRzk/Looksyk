use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::io::http::routes::to_wiki_page_url;
use crate::looksyk::model::SimplePageName;
use crate::migration::migration_1_18_0::{
    ConfigOnDiskV1_18_0, DesignOnDiskV1_18_0, JournalConfigrationOnDiskV1_18_0,
};
use crate::state::application_state::GraphRootLocation;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigOnDiskV1_20_0 {
    pub favourites: Vec<FavouriteV1_20_0>,
    pub design: DesignOnDiskV1_18_0,
    pub title: Option<String>,
    pub journal_configuration: JournalConfigrationOnDiskV1_18_0,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct FavouriteV1_20_0 {
    pub name: String,
    pub url: String,
}

pub fn migriere_1_20_0(user_application_directory: &GraphRootLocation) {
    let config_location = user_application_directory.path.join(REL_CONFIG_PATH);
    if config_location.exists() {
        println!("Found config file, migrating...");

        // Read the old config file
        let old_config_content =
            fs::read_to_string(&config_location).expect("Failed to read old config file");
        let old_config: ConfigOnDiskV1_18_0 =
            serde_json::from_str(&old_config_content).expect("Failed to parse old config file");

        // Convert to new format
        let new_config = ConfigOnDiskV1_20_0 {
            favourites: old_config
                .favourites
                .iter()
                .map(|fav| FavouriteV1_20_0 {
                    name: fav.name.name.clone(),
                    url: to_wiki_page_url(&SimplePageName {
                        name: fav.name.name.clone(),
                    }),
                })
                .collect(),
            design: old_config.design,
            title: old_config.title,
            journal_configuration: old_config.journal_configuration,
        };

        // Write the new config file
        save_config_to_file(user_application_directory, &new_config);
        println!("Migration completed successfully.");
    } else {
        println!("No shares file found, no migration needed.");
    }
}

fn save_config_to_file(data_path: &GraphRootLocation, config: &ConfigOnDiskV1_20_0) {
    let config_file_content_as_str =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    fs::write(
        crate::io::fs::config::config_path(data_path),
        config_file_content_as_str,
    )
    .unwrap();
}
