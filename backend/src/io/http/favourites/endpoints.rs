use actix_web::{delete, get, post, Responder, web};
use actix_web::web::{Data, Path};

use crate::io::fs::config::save_config_to_file;
use crate::io::http::favourites::dtos::FavListDto;
use crate::io::http::favourites::mapper::{map_from_dto, map_to_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::favourite::{add_favourite, remove_favourite, set_favourites};
use crate::state::state::AppState;

#[post("/api/favourites/{fav_name}")]
async fn insert_favourite(path: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.config.lock().unwrap();

    let new_config = add_favourite(page_name, &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    return Ok(web::Json(map_to_dto(config_guard)));
}

#[delete("/api/favourites/{fav_name}")]
async fn delete_favourite(path: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.config.lock().unwrap();

    let new_config = remove_favourite(page_name, &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    return Ok(web::Json(map_to_dto(config_guard)));
}

#[get("/api/favourites")]
async fn get_favourites(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let config_guard = data.config.lock().unwrap();

    let favs = map_to_dto(config_guard);
    Ok(web::Json(favs))
}




#[post("/api/favourites/")]
async fn update_favourites(body: web::Json<FavListDto>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_names = map_from_dto(body.into_inner());

    let mut config_guard = data.config.lock().unwrap();

    let new_config = set_favourites(page_names, &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    return Ok(web::Json(map_to_dto(config_guard)));
}


