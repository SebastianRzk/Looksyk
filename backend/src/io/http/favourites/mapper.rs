use crate::io::http::favourites::dtos::{FavDto, FavListDto};
use crate::looksyk::data::config::runtime_graph_configuration::Favourite;

pub fn map_from_dto(fav_list_dto: FavListDto) -> Vec<Favourite> {
    fav_list_dto
        .list
        .iter()
        .map(|f| Favourite {
            name: f.name.clone(),
            url: f.url.clone(),
        })
        .collect()
}

pub fn map_to_dto(favourites: &[Favourite]) -> FavListDto {
    FavListDto {
        list: favourites
            .iter()
            .map(|f| FavDto {
                name: f.name.clone(),
                url: f.url.clone(),
            })
            .collect::<Vec<FavDto>>(),
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::favourites::dtos::{FavDto, FavListDto};
    use crate::looksyk::data::config::runtime_graph_configuration::builder::page_favourite_str;

    #[test]
    fn test_map_to_dto() {
        let dto = super::map_to_dto(&[page_favourite_str("test1"), page_favourite_str("test2")]);

        assert_eq!(dto.list.len(), 2);
        assert_eq!(dto.list[0].name, "test1");
        assert_eq!(dto.list[0].url, page_favourite_str("test1").url);
        assert_eq!(dto.list[1].name, "test2");
        assert_eq!(dto.list[1].url, page_favourite_str("test2").url);
    }

    #[test]
    fn test_map_from_dto() {
        let dto = super::map_from_dto(FavListDto {
            list: vec![
                FavDto {
                    name: "test1".to_string(),
                    url: page_favourite_str("test1").url.clone(),
                },
                FavDto {
                    name: "test2".to_string(),
                    url: page_favourite_str("test2").url.clone(),
                },
            ],
        });

        assert_eq!(
            dto,
            vec![
                page_favourite_str("test1"),
                page_favourite_str("test2")
            ]
        );
    }
}
