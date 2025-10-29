use crate::io::http::page::dtos::ToValidateDto;
use crate::io::http::page::mapper::map_to_block_dto;
use crate::looksyk::model::RawBlock;
use crate::looksyk::parser::parse_block;
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_block;
use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{post, web, Responder};

#[post("/api/parse")]
async fn parse(
    content: web::Json<ToValidateDto>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let raw_block = RawBlock {
        indentation: 0,
        text_content: content.block.lines().into_iter().map(|x|x.to_string()).collect(),
    };
    let parsed_block = parse_block(&raw_block);

    let user_page_guard = data.a_user_pages.lock().unwrap();
    let journal_page_guard = data.b_journal_pages.lock().unwrap();
    let todo_index_guard = data.c_todo_index.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_guard = data.e_asset_cache.lock().unwrap();

    let serialized_block = render_block(
        &parsed_block,
        &StaticRenderContext {
            journal_pages: &journal_page_guard,
            user_pages: &user_page_guard,
            todo_index: &todo_index_guard,
            tag_index: &tag_guard,
        },
        &mut asset_guard,
        &data.data_path,
    );

    drop(user_page_guard);
    drop(todo_index_guard);
    drop(tag_guard);
    drop(asset_guard);

    Ok(web::Json(map_to_block_dto(&serialized_block)))
}
