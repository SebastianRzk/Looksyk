use crate::io::http::favourites::dtos::FavListDto;
use crate::looksyk::config::config::Favourite;
use crate::looksyk::model::SimplePageName;

pub fn map_from_dto(fav_list_dto: FavListDto) -> Vec<SimplePageName> {
    fav_list_dto
        .list
        .iter()
        .map(|f| SimplePageName { name: f.clone() })
        .collect()
}

pub fn map_to_dto(favourites: &Vec<Favourite>) -> FavListDto {
    FavListDto {
        list: favourites
            .iter()
            .map(|f| f.name.name.clone())
            .collect::<Vec<String>>(),
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::favourites::dtos::FavListDto;
    use crate::looksyk::config::config::builder::favourite_str;

    #[test]
    fn test_map_to_dto() {
        let dto = super::map_to_dto(&vec![favourite_str("test1"), favourite_str("test2")]);

        assert_eq!(dto.list.len(), 2);
        assert_eq!(dto.list[0], "test1");
        assert_eq!(dto.list[1], "test2");
    }

    #[test]
    fn test_map_from_dto() {
        let dto = super::map_from_dto(FavListDto {
            list: vec!["test1".to_string(), "test2".to_string()],
        });

        assert_eq!(dto, vec![favourite_str("test1").name, favourite_str("test2").name]);
    }
}
