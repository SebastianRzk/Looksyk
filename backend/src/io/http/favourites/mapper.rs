use crate::io::http::favourites::dtos::FavListDto;
use crate::looksyk::config::config::Config;
use crate::looksyk::model::SimplePageName;
use std::sync::MutexGuard;

pub fn map_from_dto(fav_list_dto: FavListDto) -> Vec<SimplePageName> {
    fav_list_dto
        .list
        .iter()
        .map(|f| SimplePageName { name: f.clone() })
        .collect()
}

pub fn map_to_dto(config_guard: MutexGuard<Config>) -> FavListDto {
    FavListDto {
        list: config_guard
            .favourites
            .iter()
            .map(|f| f.name.name.clone())
            .collect::<Vec<String>>(),
    }
}
