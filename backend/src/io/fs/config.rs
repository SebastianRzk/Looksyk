use crate::io::fs::basic_file::read_file;
use crate::io::fs::paths::REL_CONFIG_PATH;
use crate::looksyk::data::config::runtime_graph_configuration::{
    Appearance, Config, Design, Favourite, JournalConfigration, JournalTitleFormat,
    ShowWeekdayInTitle,
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
    pub journal_configuration: JournalConfigrationOnDisk,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct JournalConfigrationOnDisk {
    pub journal_title_format: String,
    pub show_weekday_in_title: String,
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
        journal_configuration: JournalConfigration {
            journal_title_format: JournalTitleFormat::from_str(
                &config_on_disk.journal_configuration.journal_title_format,
            )
            .expect("Failed to parse journal title format from config"),
            show_weekday_in_title: ShowWeekdayInTitle::from_str(
                &config_on_disk.journal_configuration.show_weekday_in_title,
            )
            .expect("Failed to parse show weekday in title from config"),
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
        journal_configuration: JournalConfigrationOnDisk {
            journal_title_format: config
                .journal_configuration
                .journal_title_format
                .to_string(),
            show_weekday_in_title: config
                .journal_configuration
                .show_weekday_in_title
                .to_string(),
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

pub fn config_path(data_path: &GraphRootLocation) -> PathBuf {
    data_path.path.clone().join(REL_CONFIG_PATH)
}

#[cfg(test)]
mod tests {
    use crate::io::fs::config::convert_config_to_on_disk;
    use crate::looksyk::data::config::runtime_graph_configuration::{
        Appearance, Config, Design, JournalConfigration, JournalTitleFormat, ShowWeekdayInTitle,
    };

    #[test]
    fn test_convert_config_to_on_disk() {
        let result = convert_config_to_on_disk(&Config {
            favourites: vec![],
            design: Design {
                primary_color: "blue".to_string(),
                background_color: "white".to_string(),
                foreground_color: "black".to_string(),
                primary_shading: "light".to_string(),
                appearance: Appearance::Light,
            },
            title: Some("My Graph".to_string()),
            journal_configuration: JournalConfigration {
                journal_title_format: JournalTitleFormat::World,
                show_weekday_in_title: ShowWeekdayInTitle::AsPrefix,
            },
        });

        assert_eq!(result.journal_configuration.journal_title_format, "world");
        assert_eq!(
            result.journal_configuration.show_weekday_in_title,
            "as_prefix"
        );
        assert_eq!(result.title, Some("My Graph".to_string()));
        assert_eq!(result.design.appearance, "light");
        assert_eq!(result.design.primary_color, "blue".to_string());
        assert_eq!(result.design.foreground_color, "black".to_string());
        assert_eq!(result.design.primary_shading, "light".to_string());
        assert_eq!(result.design.background_color, "white".to_string());
    }

    #[test]
    fn test_convert_config_on_disk_to_config() {
        let result = super::convert_config_on_disk_to_config(super::ConfigOnDisk {
            favourites: vec![],
            design: super::DesignOnDisk {
                primary_color: "blue".to_string(),
                background_color: "white".to_string(),
                foreground_color: "black".to_string(),
                primary_shading: "light".to_string(),
                appearance: "light".to_string(),
            },
            title: Some("My Graph".to_string()),
            journal_configuration: super::JournalConfigrationOnDisk {
                journal_title_format: "world".to_string(),
                show_weekday_in_title: "as_prefix".to_string(),
            },
        });

        assert_eq!(
            result.journal_configuration.journal_title_format,
            JournalTitleFormat::World
        );
        assert_eq!(
            result.journal_configuration.show_weekday_in_title,
            ShowWeekdayInTitle::AsPrefix
        );
        assert_eq!(result.title, Some("My Graph".to_string()));
        assert_eq!(result.design.appearance, Appearance::Light);
        assert_eq!(result.design.primary_color, "blue".to_string());
        assert_eq!(result.design.foreground_color, "black".to_string());
        assert_eq!(result.design.primary_shading, "light".to_string());
        assert_eq!(result.design.background_color, "white".to_string());
    }
}
