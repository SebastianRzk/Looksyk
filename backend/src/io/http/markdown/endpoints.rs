use actix_web::{post, web, Responder};
use actix_web::web::Data;
use crate::io::http::page::dtos::ToValidate;
use crate::io::http::page::mapper::map_to_block_dto;
use crate::looksyk::model::RawBlock;
use crate::looksyk::parser::parse_block;
use crate::looksyk::renderer::{render_block, StaticRenderContext};
use crate::state::state::AppState;

#[post("/api/parse")]
async fn parse(content: web::Json<ToValidate>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    println!("on demand render block");
    let raw_block = RawBlock {
        indentation: 0,
        text_content: vec![content.block.clone()],
    };
    let parsed_block = parse_block(&raw_block);

    let user_page_guard = data.a_user_pages.lock().unwrap();
    let todo_index_guard = data.c_todo_index.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_guard = data.e_asset_cache.lock().unwrap();

    let serialized_block = render_block(
        &parsed_block,
        &StaticRenderContext{
            user_pages: &user_page_guard,
            todo_index: &todo_index_guard,
            tag_index: &tag_guard,
        },
        &mut asset_guard,
        &data.data_path);

    drop(user_page_guard);
    drop(todo_index_guard);
    drop(tag_guard);
    drop(asset_guard);

    Ok(web::Json(map_to_block_dto(&serialized_block)))
}
