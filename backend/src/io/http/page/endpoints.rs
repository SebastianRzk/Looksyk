extern crate urlencoding;

use actix_web::web::{Data, Path};
use actix_web::{post, web, Responder, Result};

use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateBlockContentDto;
use crate::io::http::page::mapper::{map_markdown_block_dto, map_to_block_dto};
use crate::looksyk::index::index::update_index_for_file;
use crate::looksyk::model::{MarkdownReference, PageId, PageType, RawBlock, RawMarkdownFile};
use crate::looksyk::page_index::{get_page_type, strip_prefix};
use crate::looksyk::parser::{parse_block, parse_markdown_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::render_block;
use crate::looksyk::serializer::update_and_serialize_page;
use crate::state::state::{AppState, CurrentPageAssociatedState};


#[post("/api/pagesbyid/{page_id}/block/{block_number}")]
async fn update_block(path: Path<(String, usize)>, body: web::Json<UpdateBlockContentDto>, data: Data<AppState>) -> Result<impl Responder> {
    let request_body = body.into_inner();
    let (file_id, block_number) = path.into_inner();
    let page_id = PageId {
        id: file_id
    };
    let page_type = get_page_type(&page_id);
    let entity = map_markdown_block_dto(&request_body, MarkdownReference {
        block_number,
        page_name: strip_prefix(&page_id, &page_type),
        page_id: page_id.clone(),
    });
    let selected_page;


    let page_type = get_page_type(&page_id);
    let simple_page_name = strip_prefix(&page_id, &page_type);

    match page_type {
        PageType::JournalPage => {
            let journal_guard = data.journal_pages.lock().unwrap();
            selected_page = journal_guard.entries.get(&simple_page_name).unwrap().clone();
            drop(journal_guard);
        }
        PageType::UserPage => {
            let page_guard = data.user_pages.lock().unwrap();
            println!("Simple page {}", simple_page_name.name);
            selected_page = page_guard.entries.get(&simple_page_name).unwrap().clone();
            drop(page_guard);
        }
    }

    let serialized_page = update_and_serialize_page(
        &entity,
        &selected_page,
    );
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines
    });

    write_page(PageOnDisk {
        name: strip_prefix(&page_id, &page_type).name,
        content: serialized_page.join("\n"),
    }, &data.data_path,
               &page_type);


    let mut todo_guard = data.todo_index.lock().unwrap();
    let mut tag_guard = data.tag_index.lock().unwrap();
    let mut page_guard = data.user_pages.lock().unwrap();
    let mut journal_guard = data.journal_pages.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();

    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages:& page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
    };

    let new_page_associated_state = update_index_for_file(page_id, &simple_page_name, &page_type, &updated_page, current_page_associated_state);

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;

    let parsed_block = parse_block(&RawBlock {
        indentation: 0,
        text_content: vec![entity.markdown],
    });

    let rendered_block = render_block(&parsed_block, &page_guard, &todo_guard, &tag_guard, &mut asset_cache, &data.data_path);

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(asset_cache);

    Ok(web::Json(map_to_block_dto(&rendered_block)))
}

