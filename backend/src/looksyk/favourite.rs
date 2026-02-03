use crate::io::http::routes::to_wiki_page_url;
use crate::looksyk::data::config::runtime_graph_configuration::{Config, Favourite};
use crate::looksyk::model::SimplePageName;

pub fn is_favourite(name: &SimplePageName, config: &Config) -> bool {
    for favourite in &config.favourites {
        if favourite.equals(&Favourite {
            name: name.name.clone(),
            url: to_wiki_page_url(name),
        }) {
            return true;
        }
    }
    false
}

pub fn add_favourite(new_fav: Favourite, config: &Config) -> Config {
    let mut new_favourites = config.favourites.clone();
    new_favourites.push(new_fav);

    Config {
        favourites: new_favourites,
        design: config.design.clone(),
        title: config.title.clone(),
        journal_configuration: config.journal_configuration.clone(),
    }
}

pub fn set_favourites(new_favourites: Vec<Favourite>, config: &Config) -> Config {
    Config {
        favourites: new_favourites,
        design: config.design.clone(),
        title: config.title.clone(),
        journal_configuration: config.journal_configuration.clone(),
    }
}

pub fn remove_favourite(fav_to_remove: Favourite, config: &Config) -> Config {
    let mut new_favourites: Vec<Favourite> = vec![];
    for favourite in &config.favourites {
        if !favourite.equals(&fav_to_remove) {
            new_favourites.push(favourite.clone());
        }
    }

    Config {
        favourites: new_favourites,
        design: config.design.clone(),
        title: config.title.clone(),
        journal_configuration: config.journal_configuration.clone(),
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::routes::to_wiki_page_url;
    use crate::looksyk::data::config::init::graph::default_journal_configuration;
    use crate::looksyk::data::config::runtime_graph_configuration::builder::{
        config_with_fav, empty_config, empty_design, page_favourite_str,
    };
    use crate::looksyk::data::config::runtime_graph_configuration::Config;
    use crate::looksyk::favourite::{
        add_favourite, is_favourite, remove_favourite, set_favourites,
    };
    use crate::looksyk::model::SimplePageName;

    #[test]
    fn when_fav_is_set_in_config_should_return_fav() {
        let config: Config = config_with_fav("MySite");

        let result = is_favourite(
            &SimplePageName {
                name: "MySite".to_string(),
            },
            &config,
        );

        assert!(result);
    }

    #[test]
    fn when_fav_is_not_set_in_config_should_return_not_fav() {
        let result = is_favourite(
            &SimplePageName {
                name: "MySite".to_string(),
            },
            &empty_config(),
        );

        assert!(!result);
    }

    #[test]
    fn test_add_favourite() {
        let config: Config = config_with_fav("MySite");

        let result = add_favourite(page_favourite_str("MySite2"), &config);

        assert_eq!(result.favourites.len(), 2);
        assert_eq!(result.favourites.get(1).unwrap().name, "MySite2");
        assert_eq!(
            result.favourites.get(1).unwrap().url,
            to_wiki_page_url(&SimplePageName {
                name: "MySite2".to_string(),
            })
        );
    }

    #[test]
    fn test_delete_favourite() {
        let config: Config = Config {
            favourites: vec![page_favourite_str("MySite"), page_favourite_str("MySite2")],
            design: empty_design(),
            title: None,
            journal_configuration: default_journal_configuration(),
        };

        let result = remove_favourite(page_favourite_str("MySite"), &config);

        assert_eq!(result.favourites.len(), 1);
        assert_eq!(result.favourites.first().unwrap().name, "MySite2");
    }

    #[test]
    fn test_set_favourites_should_set_favourites() {
        let old_config = config_with_fav("MyOldSite");
        let result = set_favourites(vec![page_favourite_str("MyNewSite")], &old_config);

        assert_eq!(result.favourites.len(), 1);
        assert_eq!(result.favourites.first().unwrap().name, "MyNewSite");
    }
}
