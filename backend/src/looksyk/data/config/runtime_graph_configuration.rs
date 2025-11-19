use crate::looksyk::model::SimplePageName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
pub struct Config {
    pub favourites: Vec<Favourite>,
    pub design: Design,
    pub journal_configuration: JournalConfigration,
    pub title: Option<String>,
}

#[derive(Clone)]
pub struct Design {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
    pub appearance: Appearance,
}

#[derive(Clone)]
pub struct JournalConfigration {
    pub journal_title_format: JournalTitleFormat,
    pub show_weekday_in_title: ShowWeekdayInTitle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JournalTitleFormat {
    World,
    American,
    Iso,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShowWeekdayInTitle {
    AsPrefix,
    AsSuffix,
    None,
}

impl FromStr for JournalTitleFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "world" => Ok(JournalTitleFormat::World),
            "american" => Ok(JournalTitleFormat::American),
            "iso" => Ok(JournalTitleFormat::Iso),
            _ => Err(format!("Unknown journal title format: {s}")),
        }
    }
}

impl FromStr for ShowWeekdayInTitle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "as_prefix" => Ok(ShowWeekdayInTitle::AsPrefix),
            "as_suffix" => Ok(ShowWeekdayInTitle::AsSuffix),
            "none" => Ok(ShowWeekdayInTitle::None),
            _ => Err(format!("Unknown show weekday in title option: {s}")),
        }
    }
}

impl Display for JournalTitleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalTitleFormat::World => write!(f, "world"),
            JournalTitleFormat::American => write!(f, "american"),
            JournalTitleFormat::Iso => write!(f, "iso"),
        }
    }
}

impl Display for ShowWeekdayInTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShowWeekdayInTitle::AsPrefix => write!(f, "as_prefix"),
            ShowWeekdayInTitle::AsSuffix => write!(f, "as_suffix"),
            ShowWeekdayInTitle::None => write!(f, "none"),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum Appearance {
    #[default]
    Dark,
    Light,
}

impl Display for Appearance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appearance::Dark => write!(f, "dark"),
            Appearance::Light => write!(f, "light"),
        }
    }
}

impl FromStr for Appearance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(Appearance::Dark),
            "light" => Ok(Appearance::Light),
            _ => Err(format!("Unknown appearance: {s}")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Favourite {
    pub name: SimplePageName,
}

impl Favourite {
    pub fn equals_simple_name(&self, name: &SimplePageName) -> bool {
        self.name.name == name.name
    }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::data::config::init::graph::default_journal_configuration;
    use crate::looksyk::data::config::runtime_graph_configuration::{
        Appearance, Config, Design, Favourite,
    };

    pub fn favourite_str(name: &str) -> Favourite {
        Favourite {
            name: page_name_str(name),
        }
    }

    pub fn config_with_fav(fav: &str) -> Config {
        Config {
            design: empty_design(),
            favourites: vec![favourite_str(fav)],
            title: None,
            journal_configuration: default_journal_configuration(),
        }
    }

    pub fn empty_config() -> Config {
        Config {
            favourites: vec![],
            design: empty_design(),
            title: None,
            journal_configuration: default_journal_configuration(),
        }
    }

    pub fn empty_design() -> Design {
        Design {
            primary_color: "".to_string(),
            background_color: "".to_string(),
            foreground_color: "".to_string(),
            primary_shading: "".to_string(),
            appearance: Appearance::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::data::config::runtime_graph_configuration::{
        JournalTitleFormat, ShowWeekdayInTitle,
    };
    use std::str::FromStr;

    #[test]
    fn test_appearance_from_str() {
        use crate::looksyk::data::config::runtime_graph_configuration::Appearance;

        assert_eq!(Appearance::from_str("dark").unwrap(), Appearance::Dark);
        assert_eq!(Appearance::from_str("light").unwrap(), Appearance::Light);
        assert!(Appearance::from_str("unknown").is_err());
    }

    #[test]
    fn test_appearance_display() {
        use crate::looksyk::data::config::runtime_graph_configuration::Appearance;

        assert_eq!(Appearance::Dark.to_string(), "dark");
        assert_eq!(Appearance::Light.to_string(), "light");
    }

    #[test]
    fn test_journal_title_format_from_str() {
        assert_eq!(
            JournalTitleFormat::from_str("world").unwrap(),
            JournalTitleFormat::World
        );
        assert_eq!(
            JournalTitleFormat::from_str("american").unwrap(),
            JournalTitleFormat::American
        );
        assert_eq!(
            JournalTitleFormat::from_str("iso").unwrap(),
            JournalTitleFormat::Iso
        );
        assert!(JournalTitleFormat::from_str("unknown").is_err());
    }

    #[test]
    fn test_journal_title_format_display() {
        assert_eq!(JournalTitleFormat::World.to_string(), "world");
        assert_eq!(JournalTitleFormat::American.to_string(), "american");
        assert_eq!(JournalTitleFormat::Iso.to_string(), "iso");
    }

    #[test]
    fn test_show_weekday_in_title_from_str() {
        assert_eq!(
            ShowWeekdayInTitle::from_str("as_prefix").unwrap(),
            ShowWeekdayInTitle::AsPrefix
        );
        assert_eq!(
            ShowWeekdayInTitle::from_str("as_suffix").unwrap(),
            ShowWeekdayInTitle::AsSuffix
        );
        assert_eq!(
            ShowWeekdayInTitle::from_str("none").unwrap(),
            ShowWeekdayInTitle::None
        );
        assert!(ShowWeekdayInTitle::from_str("unknown").is_err());
    }

    #[test]
    fn test_show_weekday_in_title_display() {
        assert_eq!(ShowWeekdayInTitle::AsPrefix.to_string(), "as_prefix");
        assert_eq!(ShowWeekdayInTitle::AsSuffix.to_string(), "as_suffix");
        assert_eq!(ShowWeekdayInTitle::None.to_string(), "none");
    }
}
