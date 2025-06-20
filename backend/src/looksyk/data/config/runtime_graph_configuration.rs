use crate::looksyk::model::SimplePageName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub favourites: Vec<Favourite>,
    pub design: Design,
    pub appearance: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Design {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
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
    use crate::looksyk::data::config::runtime_graph_configuration::{Config, Design, Favourite};

    pub fn favourite_str(name: &str) -> Favourite {
        Favourite {
            name: page_name_str(name),
        }
    }

    pub fn config_with_fav(fav: &str) -> Config {
        Config {
            design: empty_design(),
            favourites: vec![favourite_str(fav)],
            appearance: Some("dark".to_string()),
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
