extern crate urlencoding;

use actix_web::web::{Data, Path};
use actix_web::{post, web, Responder, Result};

use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateBlockContentDto;
use crate::io::http::page::mapper::{map_markdown_block_dto, map_to_block_dto};
use crate::io::http::page_type::get_page_id_from_external_string;
use crate::looksyk::index::index_operations::update_index_for_file;
use crate::looksyk::model::{PageType, RawBlock, RawMarkdownFile};
use crate::looksyk::parser::{parse_block, parse_markdown_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_block;
use crate::looksyk::serializer::update_and_serialize_page;
use crate::state::application_state::{AppState, CurrentPageAssociatedState};
use crate::state::block::BlockReference;
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};

#[post("/api/pagesbyid/{page_id}/block/{block_number}")]
async fn update_block(
    path: Path<(String, usize)>,
    body: web::Json<UpdateBlockContentDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> Result<impl Responder> {
    let request_body = body.into_inner();
    let (file_id, block_number) = path.into_inner();
    let page_id = get_page_id_from_external_string(&file_id);
    let entity = map_markdown_block_dto(
        &request_body,
        BlockReference {
            block_number,
            page_id: page_id.clone(),
        },
    );

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();

    let selected_page = match page_id.page_type {
        PageType::JournalPage => journal_guard.entries.get(&page_id.name).unwrap().clone(),
        PageType::UserPage => {
            println!("Simple page {page_id:?}");
            page_guard.entries.get(&page_id.name).unwrap().clone()
        }
    };

    let serialized_page = update_and_serialize_page(&entity, &selected_page);
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines,
    });

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

    let parsed_block = parse_block(&RawBlock {
        indentation: 0,
        text_content: vec![entity.markdown],
    });

    let rendered_block = render_block(
        &parsed_block,
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
        },
        &mut asset_cache,
        &data.data_path,
    );

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(asset_cache);

    match page_id.page_type {
        PageType::JournalPage => document_change(
            graph_changes,
            GraphChange::journal_page_changed(page_id.name.name.clone()),
        ),
        PageType::UserPage => {
            document_change(
                graph_changes,
                GraphChange::user_page_changed(page_id.name.name.clone()),
            );
        }
    }

    Ok(web::Json(map_to_block_dto(&rendered_block)))
}
