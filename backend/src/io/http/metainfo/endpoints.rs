use actix_web::{get, Responder, web};
use actix_web::web::Data;
use serde::Serialize;
use crate::io::http::media::config::pad_url_media_location;
use crate::looksyk::model::{PageId, PageType};
use crate::looksyk::page_index::{get_page_type, strip_prefix};
use crate::state::state::AppState;

#[get("/api/metainfo/")]
async fn get_metainfo(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_guard = data.user_pages.lock().unwrap();
    let tag_guard = data.tag_index.lock().unwrap();
    let media_guard = data.media_index.lock().unwrap();


    let mut tags: Vec<String> = page_guard.entries.keys().into_iter().map(|x| x.name.clone()).collect();
    tags.extend(tag_guard.entries.keys().into_iter().map(|x| to_meta(x)).filter(|x| x.page_type == PageType::UserPage).map(|x| x.simple_name));
    tags.sort_by(|a, b| b.to_lowercase().cmp(&a.to_lowercase()));
    tags.dedup();

    let mut media: Vec<String> = media_guard.media.iter().map(|x| pad_url_media_location(&x.file_name)).collect();
    media.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    Ok(web::Json(MetaInfoDto {
        tags,
        media,
    }))
}

fn to_meta(page_id: &PageId) -> TagMeta{
    let page_type = get_page_type(page_id);
    TagMeta {
        simple_name: strip_prefix(page_id, &page_type).name.clone(),
        page_type

    }
}

struct TagMeta {
    simple_name: String,
    page_type: PageType
}

#[derive(Serialize)]
struct MetaInfoDto {
    tags: Vec<String>,
    media: Vec<String>,
}
