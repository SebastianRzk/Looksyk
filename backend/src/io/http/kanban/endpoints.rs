use crate::io::http::kanban::dtos::{GetKanbanRequestDto, KanbanDataDto};
use crate::looksyk::kanban::get_kanban_from_tag;
use crate::looksyk::kanban::models::KanbanTitle;
use crate::looksyk::kanban::renderer::render_kanban;
use crate::looksyk::model::SimplePageName;
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::state::application_state::AppState;
use crate::state::block_properties::BlockPropertyKey;
use crate::state::markdown_file::MarkdownFileIndex;
use actix_web::web::Data;
use actix_web::{post, web, Responder};

#[post("/api/kanban/")]
async fn get_kanban(
    body: web::Json<GetKanbanRequestDto>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let request_data = body.into_inner();

    let user_pages_guard = data.a_user_pages.lock().unwrap();
    let journal_guard = data.b_journal_pages.lock().unwrap();
    let todo_guard = data.c_todo_index.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let block_properties_guard = data.h_block_properties.lock().unwrap();
    let kanban = get_kanban_from_tag(
        KanbanTitle {
            title: request_data.title,
        },
        SimplePageName {
            name: request_data.tag,
        },
        BlockPropertyKey {
            value: request_data.column_identifier,
        },
        request_data
            .column_values
            .into_iter()
            .map(|v| crate::state::block_properties::BlockPropertyValue { value: v })
            .collect(),
        &block_properties_guard,
        &BlockPropertyKey {
            value: request_data.priority_identifier,
        },
        &MarkdownFileIndex {
            journal_page_index: &journal_guard,
            user_page_index: &user_pages_guard,
        },
    );

    let rendered_kanban = render_kanban(
        kanban,
        &StaticRenderContext {
            tag_index: &tag_guard,
            user_pages: &user_pages_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
        },
        &mut data.e_asset_cache.lock().unwrap(),
        &data.data_path,
    );

    drop(user_pages_guard);
    drop(journal_guard);
    drop(todo_guard);
    drop(tag_guard);
    drop(block_properties_guard);

    let dto: KanbanDataDto = rendered_kanban.into();

    Ok(web::Json(dto))
}
