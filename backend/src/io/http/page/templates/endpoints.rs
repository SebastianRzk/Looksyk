use crate::io::date::today;
use crate::io::fs::pages::{write_page, PageOnDisk};
use crate::io::http::page::mapper::map_markdown_file_to_dto;
use crate::io::http::page::templates::dtos::InsertTemplateDto;
use crate::io::http::page::templates::mapper::map_templates;
use crate::io::http::page_type::get_page_id_from_external_string;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index_operations::update_index_for_file;
use crate::looksyk::model::{PageType, ParsedMarkdownFile};
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_file;
use crate::looksyk::serializer::serialize_page;
use crate::looksyk::templates;
use crate::looksyk::templates::list::TemplateId;
use crate::looksyk::title::calculate_page_title;
use crate::state::application_state::{AppState, CurrentPageAssociatedState};
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};
use actix_web::web::{Data, Json};
use actix_web::{get, post, Responder, Result};

#[get("/api/templates")]
async fn list_all_templates(data: Data<AppState>) -> Result<impl Responder> {
    let user_page_index = data.a_user_pages.lock().unwrap();

    let templates = templates::list::list_all_templates(&user_page_index);

    drop(user_page_index);

    Ok(Json(map_templates(templates)))
}

#[post("/api/templates/insert")]
async fn insert_template_into_page(
    data: Data<AppState>,
    body: Json<InsertTemplateDto>,
    graph_changes: Data<GraphChangesState>,
) -> Result<impl Responder> {
    let insert_template_dto = body.into_inner();
    let template_id = TemplateId {
        id: insert_template_dto.template_id.clone(),
    };
    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let page_id = get_page_id_from_external_string(&insert_template_dto.page_id);
    let page_to_update = match page_id.page_type {
        PageType::UserPage => page_guard.entries.get(&page_id.name),
        PageType::JournalPage => journal_guard.entries.get(&page_id.name),
    };
    let template = page_guard.entries.get(&template_id.into());
    let empty = ParsedMarkdownFile::empty();
    let updated_page = templates::render::append_template_to_page(
        insert_template_dto.block_number,
        template.unwrap_or(&empty),
        page_to_update.unwrap_or(&empty),
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

    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_guard = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

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

    let rendered_page = render_file(
        &updated_page,
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
        },
        &mut asset_guard,
        &data.data_path,
    );

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(block_properties_guard);

    let is_fav = match page_id.page_type {
        PageType::UserPage => is_favourite(&page_id.name, &data.g_config.lock().unwrap()),
        PageType::JournalPage => false,
    };
    let title = calculate_page_title(&page_id, &config_guard.journal_configuration, today());

    match page_id.page_type {
        PageType::UserPage => {
            document_change(
                graph_changes,
                GraphChange::user_page_changed(format!(
                    "template inserted into user page: {}",
                    page_id.name.name
                )),
            );
        }
        PageType::JournalPage => {
            document_change(
                graph_changes,
                GraphChange::journal_page_changed(format!(
                    "template inserted into journal page: {}",
                    page_id.name.name
                )),
            );
        }
    }

    Ok(Json(map_markdown_file_to_dto(rendered_page, is_fav, title)))
}
