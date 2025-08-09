use crate::io::http::state::dtos::state_refreshed;
use crate::looksyk::data::graph::load_graph_data;
use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{post, web, Responder};

#[post("/api/state/refresh")]
pub async fn post_refresh_internal_state(
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    refresh_internal_state(data);
    Ok(web::Json(state_refreshed()))
}

pub fn refresh_internal_state(data: Data<AppState>) {
    let new_state = load_graph_data(&data.data_path);

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();
    let mut media_index = data.f_media_index.lock().unwrap();
    let mut config = data.g_config.lock().unwrap();

    *page_guard = new_state.a_user_pages;
    *journal_guard = new_state.b_journal_pages;
    *todo_guard = new_state.c_todo_index;
    *tag_guard = new_state.d_tag_index;
    *asset_cache = new_state.e_asset_cache;
    *media_index = new_state.f_media_index;
    *config = new_state.g_config;

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(asset_cache);
    drop(media_index);
    drop(config);
}
