use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_web::{Error, get};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::web;

const STATIC_PATH: &str = "./static";

#[get("/")]
async fn index_html() -> Result<NamedFile, Error> {
    let static_file_name = "index.html";
    let complete_path = to_static_path(static_file_name);
    println!("serving {}", complete_path.display());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/{filename}.js")]
async fn js(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.js", path.into_inner());
    let complete_path = to_static_path(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/{filename}.css")]
async fn css(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.css", path.into_inner());
    let complete_path = to_static_path(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}


#[get("/assets/fonts/{filename}.css")]
async fn font_css(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.css", path.into_inner());
    println!("serving font css {}", static_file_name.as_str());
    let complete_path = to_static_asset_fonts(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}


#[get("/assets/fonts/ebgaramond/{filename}.ttf")]
async fn font_garamond(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.ttf", path.into_inner());
    let complete_path = to_garamond(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/fonts/material-icons/material.woff2")]
async fn font_material() -> Result<NamedFile, Error> {
    let complete_path = to_static_asset_fonts("material-icons").join("material.woff2");
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/emoji/{filename}.svg")]
async fn emoji(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.svg", path.into_inner());
    println!("serving emoji {}", static_file_name.as_str());
    let complete_path = to_static_assets("emoji").join(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/assets/js/{filename}.js")]
async fn asset_js(path: web::Path<String>) -> Result<NamedFile, Error> {
    let static_file_name = format!("{}.js", path.into_inner());
    let complete_path = to_static_assets("js").join(static_file_name.as_str());
    Ok(NamedFile::open(complete_path)?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

fn to_static_path(static_file_name: &str) -> PathBuf {
    Path::new(STATIC_PATH).join(static_file_name)
}

fn to_static_assets(static_file_name: &str) -> PathBuf {
    Path::new(STATIC_PATH).join("assets").join(static_file_name)
}

fn to_static_asset_fonts(static_file_name: &str) -> PathBuf {
    Path::new(STATIC_PATH).join("assets").join("fonts").join(static_file_name)
}
fn to_garamond(static_file_name: &str) -> PathBuf {
    Path::new(STATIC_PATH).join("assets").join("fonts").join("ebgaramond").join(static_file_name)
}
