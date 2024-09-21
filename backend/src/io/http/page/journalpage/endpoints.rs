use actix_web::{get, post, web, Responder};
use actix_web::web::{Data, Path};
use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateMarkdownFileDto;
use crate::io::http::page::mapper::{map_from_update_markdown_dto, map_markdown_file_to_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index::update_index_for_file;
use crate::looksyk::model::{PageType, RawMarkdownFile};
use crate::looksyk::page_index::append_journal_page_prefix;
use crate::looksyk::parser::{parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::render_file;
use crate::looksyk::serializer::serialize_page;
use crate::state::state::AppState;

#[get("/api/journal/{journal_name}")]
async fn get_journal(path: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
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


#[post("/api/journal/{page_name}")]
async fn update_journal(path: Path<String>, body: web::Json<UpdateMarkdownFileDto>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
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