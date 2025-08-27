use std::path::{Path, PathBuf};

use crate::looksyk::data::config::theme::custom_user_theme_path;
use crate::state::application_state::AppState;
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::web::Data;
use actix_web::{get, Error};
use actix_web::{web, HttpRequest, HttpResponse};

#[get("/")]
pub async fn index_html(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    index_html_response(&state, &req)
}

fn index_html_response(state: &Data<AppState>, req: &HttpRequest) -> Result<HttpResponse, Error> {
    let static_file_name = "index.html";
    let complete_path = to_static_path(&state.static_path, static_file_name);

    let file = NamedFile::open(complete_path)?
        .use_last_modified(false)
        .use_etag(false)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        });

    let mut response = file.into_response(req);
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store, no-cache, must-revalidate"),
    );
    response
        .headers_mut()
        .insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
    response
        .headers_mut()
        .insert(header::EXPIRES, header::HeaderValue::from_static("0"));

    Ok(response)
}

pub async fn index(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    index_html_response(&state, &req)
}

#[get("/{filename}.js")]
pub async fn js(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.js", path.into_inner());
    let complete_path = to_static_path(&state.static_path, static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/{filename}.css")]
pub async fn css(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.css", path.into_inner());
    let complete_path = to_static_path(&state.static_path, static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("custom/user-theme.css")]
async fn user_css(state: Data<AppState>) -> Result<NamedFile, Error> {
    let complete_path = custom_user_theme_path(&state.data_path);
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fonts/{filename}.css")]
async fn font_css(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.css", path.into_inner());
    let complete_path = to_static_asset_fonts(&state.static_path, static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fav.png")]
async fn fav(state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = "fav.png";
    let complete_path = to_static_assets(&state.static_path, static_file_name);
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fonts/ebgaramond/{filename}.ttf")]
async fn font_garamond(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.ttf", path.into_inner());
    let complete_path = to_garamond(&state.static_path, static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fonts/noto/{filename}.ttf")]
async fn font_noto(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.ttf", path.into_inner());
    let complete_path = to_noto(&state.static_path, static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fonts/material-icons/material.woff2")]
async fn font_material(state: Data<AppState>) -> Result<NamedFile, Error> {
    let complete_path =
        to_static_asset_fonts(&state.static_path, "material-icons").join("material.woff2");
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/emoji/{filename}.svg")]
async fn emoji(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.svg", path.into_inner());
    println!("serving emoji {}", static_file_name.as_str());
    let complete_path =
        to_static_assets(&state.static_path, "emoji").join(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/js/{filename}.js")]
async fn asset_js(path: web::Path<String>, state: Data<AppState>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.js", path.into_inner());
    let complete_path = to_static_assets(&state.static_path, "js").join(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

fn to_static_path(static_path: &str, static_file_name: &str) -> PathBuf {
    Path::new(static_path).join(static_file_name)
}

fn to_static_assets(static_path: &String, static_file_name: &str) -> PathBuf {
    Path::new(static_path).join("assets").join(static_file_name)
}

fn to_static_asset_fonts(static_path: &String, static_file_name: &str) -> PathBuf {
    Path::new(static_path)
        .join("assets")
        .join("fonts")
        .join(static_file_name)
}

fn to_garamond(static_path: &String, static_file_name: &str) -> PathBuf {
    Path::new(static_path)
        .join("assets")
        .join("fonts")
        .join("ebgaramond")
        .join(static_file_name)
}

fn to_noto(static_path: &String, static_file_name: &str) -> PathBuf {
    Path::new(static_path)
        .join("assets")
        .join("fonts")
        .join("noto")
        .join(static_file_name)
}
