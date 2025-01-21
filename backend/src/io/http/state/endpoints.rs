use crate::init_data;
use crate::io::http::state::dtos::state_refreshed;
use crate::state::state::AppState;
use actix_web::web::Data;
use actix_web::{post, web, Responder};

#[post("/api/state/refresh")]
async fn update_block(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let new_state = init_data(data.data_path.clone(), data.title.clone());

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

    Ok(web::Json(state_refreshed()))
}
