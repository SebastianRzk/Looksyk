extern crate urlencoding;

use actix_web::{get, post, Responder, Result, web};
use actix_web::web::{Data, Path};

use crate::io::fs::pages::{PageOnDisk, write_page};
use crate::io::http::dtos::{ToValidate, UpdateBlockContentDto, UpdateMarkdownFileDto};
use crate::io::http::mapper::{map_from_update_markdown_dto, map_markdown_block_dto, map_markdown_file_to_dto, map_to_block_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::user_page_overview::generate_overview_page;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index::update_index_for_file;
use crate::looksyk::model::{MarkdownReference, PageId, PageType, RawBlock, RawMarkdownFile};
use crate::looksyk::page_index::{append_journal_page_prefix, get_page_type, strip_prefix};
use crate::looksyk::parser::{parse_block, parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::{render_block, render_file};
use crate::looksyk::serializer::{serialize_page, update_and_serialize_page};
use crate::state::state::AppState;

#[post("/api/parse")]
async fn parse(content: web::Json<ToValidate>, data: Data<AppState>) -> Result<impl Responder> {
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

#[get("/api/journal/{journal_name}")]
async fn get_journal(path: Path<String>, data: Data<AppState>) -> Result<impl Responder> {
    let simple_page_name = page_name(path.into_inner());
    let journal_guard = data.journal_pages.lock().unwrap();
    let page_guard = data.user_pages.lock().unwrap();
    let todo_index_guard = data.todo_index.lock().unwrap();
    let page = journal_guard.entries.get(&simple_page_name);
    let mut asset_cache = data.asset_cache.lock().unwrap();


    let fav = is_favourite(&simple_page_name, &data.config.lock().unwrap());

    if page.is_some() {
        let parsed_page = page.unwrap();
        let prepared_page = render_file(
            parsed_page,
            &page_guard,
            &todo_index_guard,
            &data.tag_index.lock().unwrap(),
            &mut asset_cache,
            &data.data_path,
        );
        return Ok(web::Json(map_markdown_file_to_dto(prepared_page, fav)));
    }

    Ok(web::Json(map_markdown_file_to_dto(render_file(
        &generate_page_not_found(), &page_guard, &todo_index_guard,
        &data.tag_index.lock().unwrap(), &mut asset_cache, &data.data_path), fav)))
}


#[get("/api/builtin-pages/user-page-overview")]
async fn get_overview_page(data: Data<AppState>) -> Result<impl Responder> {
    let tag_index_guard = data.tag_index.lock().unwrap();
    let guard = data.user_pages.lock().unwrap();
    let overview_page = generate_overview_page(&tag_index_guard, &guard);
    let todo_guard = data.todo_index.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();
    let rendered_file = render_file(&overview_page, &guard, &todo_guard, &tag_index_guard, &mut asset_cache, &data.data_path);
    Ok(web::Json(map_markdown_file_to_dto(rendered_file, false)))
}


#[post("/api/journal/{page_name}")]
async fn update_journal(path: Path<String>, body: web::Json<UpdateMarkdownFileDto>, data: Data<AppState>) -> Result<impl Responder> {
    let request_body = body.into_inner();
    let simple_page_name = page_name(path.into_inner());

    let parsed_page = parse_markdown_update_file(map_from_update_markdown_dto(request_body));
    let serialized_page = serialize_page(&parsed_page);
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines
    });

    write_page(PageOnDisk {
        name: simple_page_name.name.clone(),
        content: serialized_page.join("\n"),
    }, &data.data_path, &PageType::JournalPage);


    let mut page_guard = data.user_pages.lock().unwrap();
    let mut todo_guard = data.todo_index.lock().unwrap();
    let mut tag_guard = data.tag_index.lock().unwrap();
    let mut journal_guard = data.journal_pages.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();

    let page_id = append_journal_page_prefix(&simple_page_name);
    let (todo, tag, page, journal) = update_index_for_file(page_id, &simple_page_name, &PageType::JournalPage, &updated_page, &todo_guard, &tag_guard, &page_guard, &journal_guard);

    *todo_guard = todo;
    *tag_guard = tag;
    *page_guard = page;
    *journal_guard = journal;

    let is_fav = is_favourite(&simple_page_name, &data.config.lock().unwrap());
    let rendered_file = render_file(&updated_page, &page_guard, &todo_guard, &tag_guard, &mut asset_cache, &data.data_path);

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(asset_cache);

    return Ok(web::Json(map_markdown_file_to_dto(rendered_file, is_fav)));
}

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

    let (todo, tag, page, journal) = update_index_for_file(page_id, &simple_page_name, &page_type, &updated_page, &todo_guard, &tag_guard, &page_guard, &journal_guard);

    *todo_guard = todo;
    *tag_guard = tag;
    *page_guard = page;
    *journal_guard = journal;

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

    return Ok(web::Json(map_to_block_dto(&rendered_block)));
}

