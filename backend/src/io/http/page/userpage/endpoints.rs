use crate::io::date::today;
use crate::io::fs::pages::{delete_user_file, write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateMarkdownFileDto;
use crate::io::http::page::mapper::{map_from_update_markdown_dto, map_markdown_file_to_dto};
use crate::io::http::page::userpage::dtos::{PageDeletedDto, RenamePageDto, RenamePageResultDto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::builtinpage::user_page_overview::generate_overview_page;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index_operations::{
    remove_page_from_internal_state, update_index_for_file,
};
use crate::looksyk::index::rename::{rename_page_across_all_files, NewPageName, OldPageName};
use crate::looksyk::index::tag::render_tag_index_for_page;
use crate::looksyk::model::{PageTitle, PageType, ParsedMarkdownFile, RawMarkdownFile};
use crate::looksyk::parser::{parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_file;
use crate::looksyk::renderer::title::{calculate_user_page_title, JournalTitleCalculatorMetadata};
use crate::looksyk::serializer::serialize_page;
use crate::state::application_state::{
    AppState, CurrentPageAssociatedState, CurrentPageOnDiskState,
};
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, Responder};

#[post("/api/pages/{page_name}")]
async fn update_page(
    path: Path<String>,
    body: Json<UpdateMarkdownFileDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let request_body = body.into_inner();
    let page_name_from_input = path.into_inner();
    let page_name = page_name(page_name_from_input);

    let parsed_page = parse_markdown_update_file(map_from_update_markdown_dto(request_body));
    let serialized_page = serialize_page(&parsed_page);
    let parsed_lines = parse_lines(serialized_page.join("\n").lines());
    let updated_page = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines,
    });

    write_page(
        PageOnDisk {
            name: page_name.name.clone(),
            content: serialized_page.join("\n"),
        },
        &data.data_path,
        &PageType::UserPage,
    );

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

    let page_id = page_name.as_user_page();
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

    let is_fav = is_favourite(&page_name, &config_guard);
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
        &JournalTitleCalculatorMetadata {
            journal_configurataion: &config_guard.journal_configuration,
            today: today(),
        },
    );
    let page_title = calculate_user_page_title(&page_id);

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(block_properties_guard);

    document_change(
        graph_changes,
        GraphChange::user_page_changed(page_id.name.name),
    );

    Ok(Json(map_markdown_file_to_dto(
        rendered_file,
        is_fav,
        page_title,
    )))
}

#[get("/api/pages/{page_name}")]
async fn get_page(
    input_page_name: Path<String>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let page_name_from_input = input_page_name.into_inner();
    let simple_page_name = page_name(page_name_from_input);

    let page_guard = data.a_user_pages.lock().unwrap();
    let journal_guard = data.b_journal_pages.lock().unwrap();
    let todo_index_guard = data.c_todo_index.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();
    let is_fav = is_favourite(&simple_page_name, &config_guard);
    let page_title = calculate_user_page_title(&simple_page_name.as_user_page());

    let page = page_guard.entries.get(&simple_page_name);
    let journal_title_calculator_metadata = JournalTitleCalculatorMetadata {
        journal_configurataion: &config_guard.journal_configuration,
        today: today(),
    };

    let data_root_location = &data.data_path;
    if let Some(parsed_page) = page {
        if !parsed_page.blocks.is_empty() {
            let prepared_page = render_file(
                parsed_page,
                &StaticRenderContext {
                    user_pages: &page_guard,
                    journal_pages: &journal_guard,
                    todo_index: &todo_index_guard,
                    tag_index: &tag_guard,
                },
                &mut asset_cache,
                data_root_location,
                &journal_title_calculator_metadata,
            );
            return Ok(Json(map_markdown_file_to_dto(
                prepared_page,
                is_fav,
                page_title,
            )));
        }
    }
    let rendered_file = render_file(
        &generate_page_not_found(),
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_index_guard,
            tag_index: &tag_guard,
        },
        &mut asset_cache,
        data_root_location,
        &journal_title_calculator_metadata,
    );

    drop(page_guard);
    drop(todo_index_guard);
    drop(tag_guard);
    drop(asset_cache);

    Ok(Json(map_markdown_file_to_dto(
        rendered_file,
        is_fav,
        page_title,
    )))
}

#[get("/api/backlinks/{page_name}")]
async fn get_backlinks(
    input_page_name: Path<String>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let simple_page_name = page_name(input_page_name.into_inner());

    let page_guard = data.a_user_pages.lock().unwrap();
    let journal_guard = data.b_journal_pages.lock().unwrap();
    let todo_index_guard = data.c_todo_index.lock().unwrap();
    let tag_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache_guard = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();

    let data_root_location = &data.data_path;

    let result = render_tag_index_for_page(simple_page_name.as_user_page(), &tag_guard);

    let rendered_file = render_file(
        &result,
        &StaticRenderContext {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_index_guard,
            tag_index: &tag_guard,
        },
        &mut asset_cache_guard,
        data_root_location,
        &JournalTitleCalculatorMetadata {
            journal_configurataion: &config_guard.journal_configuration,
            today: today(),
        },
    );

    drop(page_guard);
    drop(todo_index_guard);
    drop(tag_guard);
    drop(asset_cache_guard);

    Ok(Json(map_markdown_file_to_dto(
        rendered_file,
        false,
        PageTitle::internal_page_title("".to_string()),
    )))
}

#[get("/api/builtin-pages/user-page-overview")]
async fn get_overview_page(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let user_page_guard = data.a_user_pages.lock().unwrap();
    let journal_page_guard = data.b_journal_pages.lock().unwrap();
    let todo_guard = data.c_todo_index.lock().unwrap();
    let tag_index_guard = data.d_tag_index.lock().unwrap();
    let mut asset_cache = data.e_asset_cache.lock().unwrap();
    let config_guard = data.g_config.lock().unwrap();

    let overview_page = generate_overview_page(&tag_index_guard, &user_page_guard);

    let rendered_file = render_file(
        &overview_page,
        &StaticRenderContext {
            user_pages: &user_page_guard,
            journal_pages: &journal_page_guard,
            todo_index: &todo_guard,
            tag_index: &tag_index_guard,
        },
        &mut asset_cache,
        &data.data_path,
        &JournalTitleCalculatorMetadata {
            journal_configurataion: &config_guard.journal_configuration,
            today: today(),
        },
    );

    drop(user_page_guard);
    drop(todo_guard);
    drop(tag_index_guard);
    drop(asset_cache);

    Ok(Json(map_markdown_file_to_dto(
        rendered_file,
        false,
        PageTitle::internal_page_title("User Page Overview".to_string()),
    )))
}

#[post("/api/append-page/{page_name}")]
async fn append_page(
    body: Json<UpdateMarkdownFileDto>,
    data: Data<AppState>,
    page_name_from_path: Path<String>,
    graph_change: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let request_body = body.into_inner();
    let page_name = page_name(page_name_from_path.into_inner());

    let page_appendix = parse_markdown_update_file(map_from_update_markdown_dto(request_body));
    let current_page = data
        .a_user_pages
        .lock()
        .unwrap()
        .entries
        .get(&page_name)
        .cloned();

    let merged_page = if let Some(current_page) = &current_page {
        let mut new_blocks = current_page.blocks.clone();
        new_blocks.extend(page_appendix.blocks);
        ParsedMarkdownFile { blocks: new_blocks }
    } else {
        page_appendix
    };

    write_page(
        PageOnDisk {
            name: page_name.name.clone(),
            content: serialize_page(&merged_page).join("\n"),
        },
        &data.data_path,
        &PageType::UserPage,
    );

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

    let page_id = page_name.as_user_page();
    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
        block_properties_index: &block_properties_guard,
    };
    let new_page_associated_state =
        update_index_for_file(page_id.clone(), &merged_page, current_page_associated_state);
    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;
    *block_properties_guard = new_page_associated_state.block_properties_index;

    document_change(
        graph_change,
        GraphChange::user_page_changed(page_id.name.name.clone()),
    );

    Ok("")
}

#[post("/api/rename-page")]
async fn rename_page(
    body: Json<RenamePageDto>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let body = body.into_inner();
    let old_page_name = page_name(body.old_page_name);
    let new_page_name = page_name(body.new_page_name);

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

    let current_page_associated_state = CurrentPageOnDiskState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
    };

    let rename_tag_result = rename_page_across_all_files(
        OldPageName {
            page_name: old_page_name.clone(),
        },
        NewPageName {
            page_name: new_page_name.clone(),
        },
        current_page_associated_state,
        &tag_guard,
    );

    *page_guard = rename_tag_result.new_page_associated_state.user_pages;
    *journal_guard = rename_tag_result.new_page_associated_state.journal_pages;

    for file_to_save in rename_tag_result.file_changes.changed_files {
        let page = match file_to_save.page_type {
            PageType::UserPage => page_guard.find(&file_to_save.name).unwrap(),
            PageType::JournalPage => journal_guard.find(&file_to_save.name).unwrap(),
        };

        let current_page_associated_state = CurrentPageAssociatedState {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
            block_properties_index: &block_properties_guard,
        };

        let serialized_page = serialize_page(page);
        write_page(
            PageOnDisk {
                name: file_to_save.name.name.clone(),
                content: serialized_page.join("\n"),
            },
            &data.data_path,
            &file_to_save.page_type,
        );

        let new_page_associated_state =
            update_index_for_file(file_to_save, page, current_page_associated_state);

        *todo_guard = new_page_associated_state.todo_index;
        *tag_guard = new_page_associated_state.tag_index;
        *page_guard = new_page_associated_state.user_pages;
        *journal_guard = new_page_associated_state.journal_pages;
        *block_properties_guard = new_page_associated_state.block_properties_index;
    }

    for file_to_delete in rename_tag_result.file_changes.file_to_delete {
        let current_page_associated_state = CurrentPageAssociatedState {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
            block_properties_index: &block_properties_guard,
        };

        let new_page_associated_state =
            remove_page_from_internal_state(&file_to_delete, current_page_associated_state);
        delete_user_file(&data.data_path, file_to_delete.name);

        *todo_guard = new_page_associated_state.todo_index;
        *tag_guard = new_page_associated_state.tag_index;
        *page_guard = new_page_associated_state.user_pages;
        *journal_guard = new_page_associated_state.journal_pages;
        *block_properties_guard = new_page_associated_state.block_properties_index;
    }

    drop(page_guard);
    drop(journal_guard);
    drop(todo_guard);
    drop(tag_guard);
    drop(block_properties_guard);

    document_change(
        graph_changes,
        GraphChange::page_renamed(old_page_name.name, new_page_name.name.clone()),
    );

    Ok(Json(RenamePageResultDto {
        new_page_name: new_page_name.name,
    }))
}

#[delete("/api/pages/{page_name}")]
async fn delete_page(
    input_page_name: Path<String>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let page_name_from_input = input_page_name.into_inner();
    let simple_page_name = page_name(page_name_from_input);

    let mut page_guard = data.a_user_pages.lock().unwrap();
    let mut journal_guard = data.b_journal_pages.lock().unwrap();
    let mut todo_guard = data.c_todo_index.lock().unwrap();
    let mut tag_guard = data.d_tag_index.lock().unwrap();
    let mut block_properties_guard = data.h_block_properties.lock().unwrap();

    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
        block_properties_index: &block_properties_guard,
    };

    let page_id = simple_page_name.as_user_page();
    let new_page_associated_state =
        remove_page_from_internal_state(&page_id, current_page_associated_state);
    delete_user_file(&data.data_path, simple_page_name.clone());

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;
    *block_properties_guard = new_page_associated_state.block_properties_index;

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);
    drop(block_properties_guard);

    document_change(
        graph_changes,
        GraphChange::user_page_deleted(simple_page_name.name),
    );

    Ok(Json(PageDeletedDto {}))
}
