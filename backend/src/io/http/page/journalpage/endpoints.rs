use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateMarkdownFileDto;
use crate::io::http::page::mapper::{map_from_update_markdown_dto, map_markdown_file_to_dto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::journal_overview::generate_journal_overview;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index_operations::update_index_for_file;
use crate::looksyk::model::{PageType, RawMarkdownFile};
use crate::looksyk::parser::{parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_file;
use crate::looksyk::renderer::renderer_flat::render_file_flat;
use crate::looksyk::serializer::serialize_page;
use crate::state::application_state::{AppState, CurrentPageAssociatedState};
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};
use actix_web::web::{Data, Path};
use actix_web::{get, post, web, Responder};

#[get("/api/builtin-pages/journal-overview")]
async fn journal_overview(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let journals = data.b_journal_pages.lock().unwrap();
    let journal_overview = generate_journal_overview(journals.entries.keys().cloned().collect());
    Ok(web::Json(map_markdown_file_to_dto(
        render_file_flat(&journal_overview),
        false,
    )))
}

#[get("/api/journal/{journal_name}")]
async fn get_journal(
    path: Path<String>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let simple_page_name = page_name(path.into_inner());

    let page_guard = data.a_user_pages.lock().unwrap();
    let journal_guard = data.b_journal_pages.lock().unwrap();
    let todo_index_guard = data.c_todo_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();

    let page = journal_guard.entries.get(&simple_page_name);

    let fav = is_favourite(&simple_page_name, &data.g_config.lock().unwrap());

    if page.is_some() && !page.unwrap().blocks.is_empty() {
        let parsed_page = page.unwrap();
        let prepared_page = render_file(
            parsed_page,
            &StaticRenderContext {
                user_pages: &page_guard,
                journal_pages: &journal_guard,
                todo_index: &todo_index_guard,
                tag_index: &data.d_tag_index.lock().unwrap(),
            },
            &mut asset_cache,
            &data.data_path,
        );
        return Ok(web::Json(map_markdown_file_to_dto(prepared_page, fav)));
    }

    let rendered_file = render_file(
        &generate_page_not_found(),
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_index_guard,
            tag_index: &data.d_tag_index.lock().unwrap(),
        },
        &mut asset_cache,
        &data.data_path,
    );

    drop(page_guard);
    drop(journal_guard);
    drop(todo_index_guard);
    drop(asset_cache);

    Ok(web::Json(map_markdown_file_to_dto(rendered_file, fav)))
}

#[post("/api/journal/{page_name}")]
async fn update_journal(
    path: Path<String>,
    body: web::Json<UpdateMarkdownFileDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let request_body = body.into_inner();
    let simple_page_name = page_name(path.into_inner());

    let parsed_page = parse_markdown_update_file(map_from_update_markdown_dto(request_body));
    let serialized_page = serialize_page(&parsed_page);
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines,
    });

    write_page(
        PageOnDisk {
            name: simple_page_name.name.clone(),
            content: serialized_page.join("\n"),
        },
        &data.data_path,
        &PageType::JournalPage,
    );

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();

    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
    };

    let page_id = simple_page_name.as_journal_page();
    let new_page_associated_state =
        update_index_for_file(page_id, &updated_page, current_page_associated_state);

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;

    let is_fav = is_favourite(&simple_page_name, &data.g_config.lock().unwrap());
    let rendered_file = render_file(
        &updated_page,
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

    document_change(
        graph_changes,
        GraphChange::journal_page_changed(simple_page_name.name.clone()),
    );

    Ok(web::Json(map_markdown_file_to_dto(rendered_file, is_fav)))
}
