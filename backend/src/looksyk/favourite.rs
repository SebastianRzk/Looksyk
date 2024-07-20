use crate::looksyk::config::config::{Config, Favourite};
use crate::looksyk::model::SimplePageName;

pub fn is_favourite(name: &SimplePageName, config: &Config) -> bool {
    for favourite in &config.favourites {
        if favourite.equals_simple_name(&name) {
            return true;
        }
    }
    return false;
}



pub fn add_favourite(simple_page_name: SimplePageName, config: &Config) -> Config {
    let mut new_favourites = config.favourites.clone();
    new_favourites.push(Favourite {
        name: simple_page_name
    });

    Config {
        favourites: new_favourites
    }
}

pub fn remove_favourite(simple_page_name: SimplePageName, config: &Config) -> Config {
    let mut new_favourites:Vec<Favourite> = vec![];
    for favourite in &config.favourites {
        if !favourite.equals_simple_name(&simple_page_name) {
            new_favourites.push(Favourite {
                name: favourite.name.clone()
            });
        }
    }

    Config {
        favourites: new_favourites
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::config::config::{Config, Favourite};
    use crate::looksyk::favourite::{add_favourite, is_favourite, remove_favourite};
    use crate::looksyk::model::SimplePageName;

    #[test]
    fn when_fav_is_set_in_config_should_return_fav() {
        let config: Config = Config {
            favourites: vec![Favourite {
                name: SimplePageName {
                    name: "MySite".to_string()
                }
            }],
        };

        let result = is_favourite(&SimplePageName{
            name: "MySite".to_string()
        }, &config);

        assert_eq!(result, true);
    }


    #[test]
    fn when_fav_is_not_set_in_config_should_return_not_fav() {
        let config: Config = Config {
            favourites: vec![],
        };

        let result = is_favourite(&SimplePageName{
            name: "MySite".to_string()
        }, &config);

        assert_eq!(result, false);
    }

    #[test]
    fn test_add_favourite() {
        let config: Config = Config {
            favourites: vec![Favourite {
                name: SimplePageName {
                    name: "MySite".to_string()
                }
            }],
        };

        let result = add_favourite(SimplePageName {
            name: "MySite2".to_string()
        }, &config);

        assert_eq!(result.favourites.len(), 2);
        assert_eq!(result.favourites.get(1).unwrap().name.name, "MySite2");
    }

    #[test]
    fn test_delete_favourite() {
        let config: Config = Config {
            favourites: vec![Favourite {
                name: SimplePageName {
                    name: "MySite".to_string()
                }
            }, Favourite {
                name: SimplePageName {
                    name: "MySite2".to_string()
                }
            }],
        };

        let result = remove_favourite(SimplePageName {
            name: "MySite".to_string()
        }, &config);

        assert_eq!(result.favourites.len(), 1);
        assert_eq!(result.favourites.get(0).unwrap().name.name, "MySite2");
    }
}