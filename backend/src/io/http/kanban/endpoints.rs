use crate::io::date::today;
use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::kanban::dtos::{GetKanbanRequestDto, KanbanDataDto, MoveKanbanItemRequestDto};
use crate::io::http::page::mapper::map_to_block_dto;
use crate::io::http::page_type::get_page_id_from_external_string;
use crate::looksyk::index::index_operations::update_index_for_file;
use crate::looksyk::kanban::models::KanbanTitle;
use crate::looksyk::kanban::renderer::render_kanban;
use crate::looksyk::kanban::{get_kanban_from_tag, move_kanban_card};
use crate::looksyk::model::{PageType, ParsedMarkdownFile, SimplePageName};
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_block;
use crate::looksyk::renderer::title::JournalTitleCalculatorMetadata;
use crate::looksyk::serializer::serialize_page;
use crate::state::application_state::{AppState, CurrentPageAssociatedState};
use crate::state::block_properties::{BlockPropertyKey, BlockPropertyValue};
use crate::state::markdown_file::MarkdownFileIndex;
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};
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
    let config_guard = data.g_config.lock().unwrap();
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
        &JournalTitleCalculatorMetadata {
            today: today(),
            journal_configurataion: &config_guard.journal_configuration,
        },
    );

    drop(user_pages_guard);
    drop(journal_guard);
    drop(todo_guard);
    drop(tag_guard);
    drop(block_properties_guard);

    let dto: KanbanDataDto = rendered_kanban.into();

    Ok(web::Json(dto))
}

#[post("/api/kanban/move_card")]
async fn move_card(
    body: web::Json<MoveKanbanItemRequestDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let request = body.into_inner();
    let page_id = get_page_id_from_external_string(&request.reference.file_id);

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

    let resolved_page: &ParsedMarkdownFile = match page_id.page_type {
        PageType::UserPage => page_guard.entries.get(&page_id.name).unwrap(),
        PageType::JournalPage => journal_guard.find(&page_id.name).unwrap(),
    };

    let updated_page = move_kanban_card(
        resolved_page,
        request.reference.block_number,
        &BlockPropertyKey { value: request.key },
        &BlockPropertyValue {
            value: request.from.clone(),
        },
        &BlockPropertyValue {
            value: request.to.clone(),
        },
    );

    let serialized_page = serialize_page(&updated_page);

    write_page(
        PageOnDisk {
            name: page_id.name.name.clone(),
            content: serialized_page.join("\n"),
        },
        &data.data_path,
        &page_id.page_type,
    );

    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
        block_properties_index: &block_properties_guard,
    };

    let new_page_associated_state = update_index_for_file(
        page_id.clone(),
        &updated_page,
        current_page_associated_state,
    );

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;
    *block_properties_guard = new_page_associated_state.block_properties_index;

    let parsed_block = updated_page.block(request.reference.block_number).unwrap();

    let rendered_block = render_block(
        parsed_block,
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
        },
        &mut asset_cache,
        &data.data_path,
        &JournalTitleCalculatorMetadata {
            today: today(),
            journal_configurataion: &config_guard.journal_configuration,
        },
    );

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(asset_cache);
    drop(block_properties_guard);

    document_change(
        graph_changes,
        GraphChange::kanban_item_moved(
            page_id.name.name,
            request.reference.block_number,
            request.from,
            request.to,
        ),
    );

    Ok(web::Json(map_to_block_dto(&rendered_block)))
}
