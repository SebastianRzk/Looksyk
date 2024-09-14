use std::fs;
use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::form::{json::Json as MPJson, MultipartForm, tempfile::TempFile};
use actix_web::{Error, error, get, HttpRequest, post, Responder};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

use crate::io::fs::basic_file::read_binary_file;
use crate::io::fs::media::{destination_path, LoadedMedia, read_media_file, write_media_config, read_file_sizes};
use crate::io::hash::hash_file_content;
use crate::io::http::mapper::map_markdown_file_to_dto;
use crate::io::http::media::config::create_media_location;
use crate::io::http::media::mapper::map_to_dto;
use crate::looksyk::builtinpage::assets_overview::generate_assets_overview_page;
use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::index::media::{find_file_by_hash, IndexedMedia};
use crate::looksyk::media::autodetect::inver_markdown_media_link;
use crate::looksyk::media::suggestion::get_suggestion_for_file;
use crate::looksyk::renderer::render_file;
use crate::state::state::AppState;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MPJson<Metadata>,
}

#[post("/api/media")]
pub async fn post_file(MultipartForm(form): MultipartForm<UploadForm>, app_state: Data<AppState>) -> actix_web::Result<impl Responder> {
    let filename = form.json.name.clone();
    println!(
        "Uploaded file {}, with size: {}",
        filename, form.file.size
    );
    println!("path {}", form.file.file.path().display());
    let file = read_binary_file(form.file.file.path().to_path_buf());

    let hash = hash_file_content(LoadedMedia {
        content: file.clone(),
    });

    let mut media_guard = app_state.media_index.lock().unwrap();

    let index_element = find_file_by_hash(&hash, &media_guard).unwrap_or_else(|| {
        let absolute_destination_path = destination_path(filename.as_str(), &app_state.data_path);
        let name = absolute_destination_path.file_name().unwrap().to_os_string().to_str().unwrap().to_string();
        let new_entry = IndexedMedia {
            file_name: create_media_location(&name),
            sha3: hash.clone(),
        };
        media_guard.media.push(new_entry.clone());
        write_media_config(&app_state.data_path, &media_guard);
        fs::write(Path::new(&absolute_destination_path), &file).unwrap();
        write_media_config(&app_state.data_path, &media_guard);
        new_entry
    });

    Ok(Json(FileUploadResult {
        inline_markdown: inver_markdown_media_link(&index_element.file_name)
    }))
}


#[get("/api/assets/suggestion/{filename:.*}")]
pub async fn asset_suggestion(req: HttpRequest) -> error::Result<impl Responder> {
    let file_name: String = req.match_info().query("filename").parse().unwrap();
    let result = get_suggestion_for_file(&AssetDescriptor::new(file_name));
    let dto = map_to_dto(result);
    Ok(Json(dto))
}


#[get("/api/builtin-pages/assets-overview")]
pub async fn assets_overview(data: Data<AppState>) -> error::Result<impl Responder> {
    let file_sizes = read_file_sizes(&data.data_path);
    let media_index = data.media_index.lock().unwrap();

    let assets_overview = generate_assets_overview_page(&media_index, file_sizes);

    let tag_index_guard = data.tag_index.lock().unwrap();
    let guard = data.user_pages.lock().unwrap();
    let todo_guard = data.todo_index.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();
    let rendered_file = render_file(&assets_overview, &guard, &todo_guard, &tag_index_guard, &mut asset_cache, &data.data_path);
    Ok(Json(map_markdown_file_to_dto(rendered_file, false)))
}


#[get("/assets/{filename:.*}")]
pub async fn assets(req: HttpRequest, data: Data<AppState>) -> Result<NamedFile, Error> {
    let path: String = req.match_info().query("filename").parse().unwrap();
    let file = read_media_file(&path, &data.data_path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileUploadResult {
    inline_markdown: String,
}