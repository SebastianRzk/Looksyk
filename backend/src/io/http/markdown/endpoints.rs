use actix_web::{post, web, Responder};
use actix_web::web::Data;
use crate::io::http::page::dtos::ToValidate;
use crate::io::http::page::mapper::map_to_block_dto;
use crate::looksyk::model::RawBlock;
use crate::looksyk::parser::parse_block;
use crate::looksyk::renderer::render_block;
use crate::state::state::AppState;

#[post("/api/parse")]
async fn parse(content: web::Json<ToValidate>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    println!("on demand render block");
    let raw_block = RawBlock {
        indentation: 0,
        text_content: vec![content.block.clone()],
    };
    let parsed_block = parse_block(&raw_block);
    let serialized_block = render_block(
        &parsed_block,
        &data.user_pages.lock().unwrap(),
        &data.todo_index.lock().unwrap(),
        &data.tag_index.lock().unwrap(),
        &mut data.asset_cache.lock().unwrap(),
        &data.data_path);
    Ok(web::Json(map_to_block_dto(&serialized_block)))
}
