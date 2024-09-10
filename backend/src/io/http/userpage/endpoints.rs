use actix_web::{get, post, Responder, web};
use actix_web::web::{Data, Path};

use crate::io::fs::pages::{PageOnDisk, write_page};
use crate::io::http::dtos::UpdateMarkdownFileDto;
use crate::io::http::mapper::{map_from_update_markdown_dto, map_markdown_file_to_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index::update_index_for_file;
use crate::looksyk::index::tag::render_tag_index_for_page;
use crate::looksyk::model::{decode_page_name, PageType, RawMarkdownFile};
use crate::looksyk::page_index::append_user_page_prefix;
use crate::looksyk::parser::{parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::render_file;
use crate::looksyk::serializer::serialize_page;
use crate::state::state::AppState;

#[post("/api/pages/{page_name}")]
async fn update_page(path: Path<String>, body: web::Json<UpdateMarkdownFileDto>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let request_body = body.into_inner();
    let page_name = decode_page_name(path.into_inner());

    let parsed_page = parse_markdown_update_file(map_from_update_markdown_dto(request_body));
    let serialized_page = serialize_page(&parsed_page);
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines
    });

    write_page(PageOnDisk {
        name: page_name.name.clone(),
        content: serialized_page.join("\n"),
    }, &data.data_path, &PageType::UserPage);


    let mut page_guard = data.user_pages.lock().unwrap();
    let mut todo_guard = data.todo_index.lock().unwrap();
    let mut tag_guard = data.tag_index.lock().unwrap();
    let mut journal_guard = data.journal_pages.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();

    let page_id = append_user_page_prefix(&page_name);
    let (todo, tag, page, journal) = update_index_for_file(page_id, &page_name, &PageType::UserPage, &updated_page, &todo_guard, &tag_guard, &page_guard, &journal_guard);

    *todo_guard = todo;
    *tag_guard = tag;
    *page_guard = page;
    *journal_guard = journal;

    let is_fav = is_favourite(&page_name, &data.config.lock().unwrap());
    let rendered_file = render_file(&updated_page, &page_guard, &todo_guard, &tag_guard, &mut asset_cache, &data.data_path);

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);

    return Ok(web::Json(map_markdown_file_to_dto(rendered_file, is_fav)));
}


#[get("/api/pages/{page_name}")]
async fn get_page(input_page_name: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let simple_page_name = decode_page_name(input_page_name.into_inner());

    let page_guard = data.user_pages.lock().unwrap();
    let todo_index_guard = data.todo_index.lock().unwrap();
    let tag_guard = data.tag_index.lock().unwrap();
    let page = page_guard.entries.get(&simple_page_name);
    let mut asset_cache = data.asset_cache.lock().unwrap();
    let is_fav = is_favourite(&simple_page_name, &data.config.lock().unwrap());
    let data_root_location = &data.data_path;
    if page.is_some() {
        let parsed_page = page.unwrap();
        let prepared_page = render_file(parsed_page, &page_guard, &todo_index_guard, &tag_guard, &mut asset_cache, data_root_location);
        return Ok(web::Json(map_markdown_file_to_dto(prepared_page, is_fav)));
    }
    Ok(web::Json(map_markdown_file_to_dto(render_file(
        &generate_page_not_found(), &page_guard, &todo_index_guard,
        &tag_guard, &mut asset_cache, data_root_location), is_fav)))
}


#[get("/api/backlinks/{page_name}")]
async fn get_backlinks(input_page_name: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let simple_page_name = page_name(input_page_name.into_inner().replace('/', "%2F"));

    let tag_guard = data.tag_index.lock().unwrap();
    let page_guard = data.user_pages.lock().unwrap();
    let todo_index_guard = data.todo_index.lock().unwrap();
    let mut asset_cache_guard = data.asset_cache.lock().unwrap();
    let data_root_location = &data.data_path;

    let result = render_tag_index_for_page(append_user_page_prefix(&simple_page_name), &tag_guard);


    Ok(web::Json(map_markdown_file_to_dto(render_file(
        &result, &page_guard, &todo_index_guard, &tag_guard, &mut asset_cache_guard, data_root_location,
    ), false)))
}
