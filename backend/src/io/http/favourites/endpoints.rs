use actix_web::web::{Data, Path, Query};
use actix_web::{delete, get, post, web, Responder};

use crate::io::fs::config::save_config_to_file;
use crate::io::http::favourites::dtos::{FavListDto, FavUrlDto};
use crate::io::http::favourites::mapper::{map_from_dto, map_to_dto};
use crate::io::http::routes::to_wiki_page_url;
use crate::looksyk::builder::page_name;
use crate::looksyk::data::config::runtime_graph_configuration::Favourite;
use crate::looksyk::favourite::{add_favourite, remove_favourite, set_favourites};
use crate::state::application_state::AppState;
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};

#[post("/api/favourites/other/{fav_name}")]
async fn insert_other_favourite(
    path: Path<String>,
    query: Query<FavUrlDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = add_favourite(
        Favourite {
            name: path.into_inner(),
            url: query.url.clone(),
        },
        &config_guard,
    );
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed(format!("favourite added: {}", query.url)),
    );
    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}



#[delete("/api/favourites/other/{fav_name}")]
async fn delete_other_favourite(
    path: Path<String>,
    query: Query<FavUrlDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = remove_favourite(
        Favourite {
            name: path.into_inner(),
            url: query.url.clone(),
        },
        &config_guard,
    );
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed(format!("favourite removed: {}", query.url)),
    );

    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}


#[post("/api/favourites/page/{fav_name}")]
async fn insert_page_favourite(
    path: Path<String>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = add_favourite(
        Favourite {
            name: page_name.name.clone(),
            url: to_wiki_page_url(&page_name),
        },
        &config_guard,
    );
    save_config_to_file(&data.data_path, &new_config);

    *config_guard = new_config;

    document_change(
        graph_changes,
        GraphChange::configuration_changed(format!("favourite added: {}", page_name.name)),
    );

    Ok(web::Json(map_to_dto(&config_guard.favourites)))
}

#[delete("/api/favourites/page/{fav_name}")]
async fn delete_favourite_page(
    path: Path<String>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_name = page_name(path.into_inner());

    let mut config_guard = data.g_config.lock().unwrap();

    let new_config = remove_favourite(
        Favourite {
            name: page_name.name.clone(),
            url: to_wiki_page_url(&page_name),
        },
        &config_guard,
    );
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
