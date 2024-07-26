use actix_web::{get, Responder, web};
use actix_web::web::Data;
use serde::Serialize;

use crate::state::state::AppState;

#[get("/api/metainfo/")]
async fn get_metainfo(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_guard = data.user_pages.lock().unwrap();
    let media_guard = data.media_index.lock().unwrap();


    let mut tags: Vec<String> = page_guard.entries.keys().into_iter().map(|x| x.name.clone()).collect();
    tags.sort_by(|a, b| b.to_lowercase().cmp(&a.to_lowercase()));

    let mut media: Vec<String> = media_guard.media.iter().map(|x| x.relative_path.clone()).collect();
    media.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    Ok(web::Json(MetaInfoDto {
        tags,
        media,
    }))
}

#[derive(Serialize)]
struct MetaInfoDto {
    tags: Vec<String>,
    media: Vec<String>,
}
