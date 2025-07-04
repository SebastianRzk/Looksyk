use crate::io::http::page::mapper::map_markdown_file_to_dto;
use crate::looksyk::builtinpage::help_page::help_page;
use actix_web::web::Json;
use actix_web::{get, Responder};

#[get("/api/builtin-pages/help")]
async fn help() -> actix_web::Result<impl Responder> {
    Ok(Json(map_markdown_file_to_dto(help_page(), false)))
}
