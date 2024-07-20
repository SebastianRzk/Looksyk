use serde::{Deserialize, Serialize};
use crate::looksyk::model::SimplePageName;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub favourites: Vec<Favourite>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Favourite{
    pub name: SimplePageName
}

impl Favourite {
    pub fn equals_simple_name(&self, name: &SimplePageName) -> bool {
        self.name.name == name.name
    }
}