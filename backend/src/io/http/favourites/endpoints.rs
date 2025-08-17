use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, web, Responder};

use crate::io::fs::config::save_config_to_file;
use crate::io::http::favourites::dtos::FavListDto;
use crate::io::http::favourites::mapper::{map_from_dto, map_to_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::favourite::{add_favourite, remove_favourite, set_favourites};
use crate::state::application_state::AppState;
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};

#[post("/api/favourites/{fav_name}")]
async fn insert_favourite(
    path: Path<String>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = add_favourite(page_name.clone(), &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed(format!("favourite added: {}", page_name.name)),
    );

    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}

#[delete("/api/favourites/{fav_name}")]
async fn delete_favourite(
    path: Path<String>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = remove_favourite(page_name.clone(), &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed(format!("favourite removed: {}", page_name.name)),
    );

    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}

#[get("/api/favourites")]
async fn get_favourites(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let config_guard = data.g_config.lock().unwrap();

    let favs = map_to_dto(&config_guard.favourites);
    Ok(web::Json(favs))
}

#[post("/api/favourites/")]
async fn update_favourites(
    body: web::Json<FavListDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_names = map_from_dto(body.into_inner());

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = set_favourites(page_names, &config_guard);
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed("favourites reordered".to_string()),
    );

    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}
