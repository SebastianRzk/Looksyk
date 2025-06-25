use crate::looksyk::model::SimplePageName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
pub struct Config {
    pub favourites: Vec<Favourite>,
    pub design: Design,
    pub appearance: Appearance,
    pub title: Option<String>,
}

#[derive(Clone)]
pub struct Design {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
}

#[derive(Clone, Default)]
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
            _ => Err(format!("Unknown appearance: {}", s)),
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
            appearance: Appearance::default(),
            title: None,
        }
    }

    pub fn empty_config() -> Config {
        Config {
            favourites: vec![],
            design: empty_design(),
            appearance: Appearance::default(),
            title: None,
        }
    }

    pub fn empty_design() -> Design {
        Design {
            primary_color: "".to_string(),
            background_color: "".to_string(),
            foreground_color: "".to_string(),
            primary_shading: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_appearance_from_str() {
        use crate::looksyk::data::config::runtime_graph_configuration::Appearance;

        assert_eq!(Appearance::from_str("dark").unwrap().to_string(), "dark");
        assert_eq!(Appearance::from_str("light").unwrap().to_string(), "light");
        assert!(Appearance::from_str("unknown").is_err());
    }

    #[test]
    fn test_appearance_display() {
        use crate::looksyk::data::config::runtime_graph_configuration::Appearance;

        assert_eq!(Appearance::Dark.to_string(), "dark");
        assert_eq!(Appearance::Light.to_string(), "light");
    }
}
