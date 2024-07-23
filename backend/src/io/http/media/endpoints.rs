use std::fs;
use std::path::Path;
use actix_files::NamedFile;
use actix_multipart::form::{json::Json as MPJson, MultipartForm, tempfile::TempFile};
use actix_web::{Error, get, HttpRequest, post, Responder};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

use crate::io::fs::basic_file::read_binary_file;
use crate::io::fs::media::{destination_path, LoadedMedia, write_media_config};
use crate::io::hash::hash_file_content;
use crate::looksyk::index::media::{find_file_by_hash, IndexedMedia};
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
        let destination_path = destination_path(filename.as_str(), &app_state.data_path);
        let name = destination_path.to_str().unwrap().to_string();
        println!("Add media object to index: {} , {}", filename.as_str(), name);
        let new_entry = IndexedMedia {
            relative_path: name,
            sha3: hash.clone(),
        };
        media_guard.media.push(new_entry.clone());
        write_media_config(&app_state.data_path, &media_guard);
        fs::write(Path::new(&destination_path), &file).unwrap();
        write_media_config(&app_state.data_path, &media_guard);
        new_entry
    });

    let filename = Path::new(&index_element.relative_path).file_name().unwrap().to_str().unwrap().to_string();


    Ok(Json(FileUploadResult {
        inline_markdown: format!("![{}](/assets/{})", filename, filename)
    }))
}



#[get("/assets/{filename:.*}")]
async fn assets(req: HttpRequest, data: Data<AppState>) -> Result<NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let complete_path = Path::new(&data.data_path.path).join("assets").join(path);
    let file = NamedFile::open(complete_path)?;
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