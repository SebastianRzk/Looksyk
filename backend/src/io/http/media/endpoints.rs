use std::fs;
use std::path::Path;
use actix_files::NamedFile;
use actix_multipart::form::{json::Json as MPJson, MultipartForm, tempfile::TempFile};
use actix_web::{Error, get, HttpRequest, post, Responder};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

use crate::io::fs::basic_file::read_binary_file;
use crate::io::fs::media::{destination_path, LoadedMedia, read_media_file, write_media_config};
use crate::io::hash::hash_file_content;
use crate::io::http::media::config::pad_url_media_location;
use crate::looksyk::index::media::{find_file_by_hash, IndexedMedia};
use crate::looksyk::media::autodetect::inver_markdown_media_link;
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
            file_name: pad_url_media_location(&name),
            sha3: hash.clone(),
        };
        media_guard.media.push(new_entry.clone());
        write_media_config(&app_state.data_path, &media_guard);
        fs::write(Path::new(&absolute_destination_path), &file).unwrap();
        write_media_config(&app_state.data_path, &media_guard);
        new_entry
    });

    Ok(Json(FileUploadResult {
        inline_markdown: inver_markdown_media_link(&index_element.file_name, &pad_url_media_location(&index_element.file_name))
    }))
}



#[get("/assets/{filename:.*}")]
async fn assets(req: HttpRequest, data: Data<AppState>) -> Result<NamedFile, Error> {
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