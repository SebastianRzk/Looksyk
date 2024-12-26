use crate::io::http::metainfo::dtos::{MetaInfoDto, TitleDto};
use crate::looksyk::model::{PageId, PageType};
use crate::looksyk::page_index::{get_page_type, strip_prefix};
use crate::state::state::AppState;
use actix_web::web::Data;
use actix_web::{get, web, Responder};
use std::collections::HashSet;

#[get("/api/title")]
async fn get_title(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(TitleDto {
        title: data.title.clone(),
    }))
}

#[get("/api/metainfo/")]
async fn get_metainfo(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_guard = data.a_user_pages.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let media_guard = data.f_media_index.lock().unwrap();

    let mut tags: Vec<String> = page_guard
        .entries
        .keys()
        .into_iter()
        .map(|x| x.name.clone())
        .collect();
    tags.extend(
        tag_guard
            .entries
            .keys()
            .into_iter()
            .map(|x| to_meta(x))
            .filter(|x| x.page_type == PageType::UserPage)
            .map(|x| x.simple_name),
    );
    tags = tags
        .iter()
        .map(|s| s.replace("%2F", "/"))
        .collect::<Vec<String>>();
    let tags_set: HashSet<String> = HashSet::from_iter(tags.into_iter());
    tags = Vec::from_iter(tags_set.into_iter());
    tags.sort_by(|a, b| a.len().cmp(&b.len()));
    tags.dedup();

    let mut media: Vec<String> = media_guard
        .media
        .iter()
        .map(|x| x.file_name.clone())
        .collect();
    media.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    drop(page_guard);
    drop(tag_guard);
    drop(media_guard);

    Ok(web::Json(MetaInfoDto { tags, media }))
}

fn to_meta(page_id: &PageId) -> TagMeta {
    let page_type = get_page_type(page_id);
    TagMeta {
        simple_name: strip_prefix(page_id, &page_type).name.clone(),
        page_type,
    }
}

struct TagMeta {
    simple_name: String,
    page_type: PageType,
}
