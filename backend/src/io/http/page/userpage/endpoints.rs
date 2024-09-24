use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, web, Responder};

use crate::io::fs::pages::{delete_user_file, write_page, PageOnDisk};
use crate::io::http::page::dtos::UpdateMarkdownFileDto;
use crate::io::http::page::mapper::{map_from_update_markdown_dto, map_markdown_file_to_dto};
use crate::io::http::page::userpage::dtos::{PageDeletedDto, RenamePageDto, RenamePageResultDto};
use crate::looksyk::builder::page_name;
use crate::looksyk::builtinpage::page_not_found::generate_page_not_found;
use crate::looksyk::builtinpage::user_page_overview::generate_overview_page;
use crate::looksyk::favourite::is_favourite;
use crate::looksyk::index::index::{remove_page_from_internal_state, update_index_for_file};
use crate::looksyk::index::rename::{rename_page_across_all_files, NewPageName, OldPageName};
use crate::looksyk::index::tag::render_tag_index_for_page;
use crate::looksyk::model::{PageType, RawMarkdownFile};
use crate::looksyk::page_index::{append_user_page_prefix, get_page_type, strip_prefix, strip_user_page_prefix};
use crate::looksyk::parser::{parse_markdown_file, parse_markdown_update_file};
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::render_file;
use crate::looksyk::serializer::serialize_page;
use crate::state::state::{AppState, CurrentPageAssociatedState, CurrentPageOnDiskState};

#[post("/api/pages/{page_name}")]
async fn update_page(path: Path<String>, body: web::Json<UpdateMarkdownFileDto>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let request_body = body.into_inner();
    let page_name_from_input = path.into_inner();
    let page_name = page_name(page_name_from_input);

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
    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
    };

    let new_page_associated_state = update_index_for_file(page_id, &page_name, &PageType::UserPage, &updated_page, current_page_associated_state);

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;

    let is_fav = is_favourite(&page_name, &data.config.lock().unwrap());
    let rendered_file = render_file(&updated_page, &page_guard, &todo_guard, &tag_guard, &mut asset_cache, &data.data_path);

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);

    Ok(Json(map_markdown_file_to_dto(rendered_file, is_fav)))
}


#[get("/api/pages/{page_name}")]
async fn get_page(input_page_name: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_name_from_input = input_page_name.into_inner();
    let simple_page_name = page_name(page_name_from_input);

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
        return Ok(Json(map_markdown_file_to_dto(prepared_page, is_fav)));
    }
    Ok(Json(map_markdown_file_to_dto(render_file(
        &generate_page_not_found(), &page_guard, &todo_index_guard,
        &tag_guard, &mut asset_cache, data_root_location), is_fav)))
}


#[get("/api/backlinks/{page_name}")]
async fn get_backlinks(input_page_name: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let simple_page_name = page_name(input_page_name.into_inner());

    let tag_guard = data.tag_index.lock().unwrap();
    let page_guard = data.user_pages.lock().unwrap();
    let todo_index_guard = data.todo_index.lock().unwrap();
    let mut asset_cache_guard = data.asset_cache.lock().unwrap();
    let data_root_location = &data.data_path;

    let result = render_tag_index_for_page(append_user_page_prefix(&simple_page_name), &tag_guard);


    Ok(Json(map_markdown_file_to_dto(render_file(
        &result, &page_guard, &todo_index_guard, &tag_guard, &mut asset_cache_guard, data_root_location,
    ), false)))
}


#[get("/api/builtin-pages/user-page-overview")]
async fn get_overview_page(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let tag_index_guard = data.tag_index.lock().unwrap();
    let guard = data.user_pages.lock().unwrap();
    let overview_page = generate_overview_page(&tag_index_guard, &guard);
    let todo_guard = data.todo_index.lock().unwrap();
    let mut asset_cache = data.asset_cache.lock().unwrap();
    let rendered_file = render_file(&overview_page, &guard, &todo_guard, &tag_index_guard, &mut asset_cache, &data.data_path);
    Ok(Json(map_markdown_file_to_dto(rendered_file, false)))
}

#[post("/api/rename-page")]
async fn rename_page(body: web::Json<RenamePageDto>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let body = body.into_inner();
    let old_page_name = page_name(body.old_page_name);
    let new_page_name = page_name(body.new_page_name);

    let mut page_guard = data.user_pages.lock().unwrap();
    let mut todo_guard = data.todo_index.lock().unwrap();
    let mut tag_guard = data.tag_index.lock().unwrap();
    let mut journal_guard = data.journal_pages.lock().unwrap();

    let current_page_associated_state = CurrentPageOnDiskState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
    };


    let rename_tag_result = rename_page_across_all_files(
        OldPageName {
            page_name: old_page_name,
        }, NewPageName {
            page_name: new_page_name.clone(),
        }, current_page_associated_state, &tag_guard);


    *page_guard = rename_tag_result.new_page_associated_state.user_pages;
    *journal_guard = rename_tag_result.new_page_associated_state.journal_pages;


    for file_to_save in rename_tag_result.file_changes.changed_files {
        eprintln!("updating file: {}", file_to_save.id);
        let page_type = get_page_type(&file_to_save);
        let simple_page_name = strip_prefix(&file_to_save, &page_type);
        let page;
        match page_type {
            PageType::UserPage => {
                page = page_guard.entries.get(&simple_page_name).unwrap();
            }
            PageType::JournalPage => {
                page = journal_guard.entries.get(&simple_page_name).unwrap();
            }
        }

        let current_page_associated_state = CurrentPageAssociatedState {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
        };

        let serialized_page = serialize_page(&page);
        write_page(PageOnDisk {
            name: simple_page_name.name.clone(),
            content: serialized_page.join("\n"),
        }, &data.data_path, &page_type);

        let new_page_associated_state = update_index_for_file(file_to_save, &simple_page_name, &page_type, &page, current_page_associated_state);


        *todo_guard = new_page_associated_state.todo_index;
        *tag_guard = new_page_associated_state.tag_index;
        *page_guard = new_page_associated_state.user_pages;
        *journal_guard = new_page_associated_state.journal_pages;
    }



    for file_to_delete in rename_tag_result.file_changes.file_to_delete {
        let simple_page_name = strip_user_page_prefix(&file_to_delete);
        let current_page_associated_state = CurrentPageAssociatedState {
            user_pages: &page_guard,
            journal_pages: &journal_guard,
            todo_index: &todo_guard,
            tag_index: &tag_guard,
        };

        let new_page_associated_state = remove_page_from_internal_state(&file_to_delete, &PageType::UserPage, &simple_page_name, current_page_associated_state);
        delete_user_file(&data.data_path, simple_page_name);

        *todo_guard = new_page_associated_state.todo_index;
        *tag_guard = new_page_associated_state.tag_index;
        *page_guard = new_page_associated_state.user_pages;
        *journal_guard = new_page_associated_state.journal_pages;
    }




    Ok(Json(RenamePageResultDto {
        new_page_name: new_page_name.name,
    }))
}

#[delete("/api/pages/{page_name}")]
async fn delete_page(input_page_name: Path<String>, data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let page_name_from_input = input_page_name.into_inner();
    let simple_page_name = page_name(page_name_from_input);

    let mut page_guard = data.user_pages.lock().unwrap();
    let mut todo_guard = data.todo_index.lock().unwrap();
    let mut tag_guard = data.tag_index.lock().unwrap();
    let mut journal_guard = data.journal_pages.lock().unwrap();

    let current_page_associated_state = CurrentPageAssociatedState {
        user_pages: &page_guard,
        journal_pages: &journal_guard,
        todo_index: &todo_guard,
        tag_index: &tag_guard,
    };

    let page_id = append_user_page_prefix(&simple_page_name);
    let new_page_associated_state = remove_page_from_internal_state(&page_id, &PageType::UserPage, &simple_page_name, current_page_associated_state);
    delete_user_file(&data.data_path, simple_page_name);

    *todo_guard = new_page_associated_state.todo_index;
    *tag_guard = new_page_associated_state.tag_index;
    *page_guard = new_page_associated_state.user_pages;
    *journal_guard = new_page_associated_state.journal_pages;

    drop(todo_guard);
    drop(tag_guard);
    drop(page_guard);
    drop(journal_guard);

    Ok(Json(PageDeletedDto{}))
}