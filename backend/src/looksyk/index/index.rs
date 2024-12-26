use crate::looksyk::index::tag::{create_tag_index_file, remove_file_from_tag_index};
use crate::looksyk::index::todo::{create_todo_index_file, remove_file_from_todo_index};
use crate::looksyk::index::userpage::{
    remove_file_from_journal_index, remove_file_from_page_index,
};
use crate::looksyk::model::{PageId, PageType, ParsedMarkdownFile, SimplePageName};
use crate::state::state::{CurrentPageAssociatedState, NewPageAssociatedState};
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;

pub fn update_index_for_file(
    file_id: PageId,
    simple_page_name: &SimplePageName,
    page_type: &PageType,
    update: &ParsedMarkdownFile,
    page_associated_state: CurrentPageAssociatedState,
) -> NewPageAssociatedState {
    let mut new_page_associated_state = remove_page_from_internal_state(
        &file_id,
        &page_type,
        &simple_page_name,
        page_associated_state,
    );

    let mut todo_index_entries = new_page_associated_state.todo_index.entries;
    create_todo_index_file(
        &mut todo_index_entries,
        &file_id,
        page_type,
        &simple_page_name,
        update,
    );

    let mut tag_index_entries = new_page_associated_state.tag_index.entries;
    create_tag_index_file(&mut tag_index_entries, &file_id, &update);

    match page_type {
        PageType::UserPage => {
            new_page_associated_state
                .user_pages
                .entries
                .insert(simple_page_name.clone(), update.clone());
        }
        PageType::JournalPage => {
            new_page_associated_state
                .journal_pages
                .entries
                .insert(simple_page_name.clone(), update.clone());
        }
    }

    NewPageAssociatedState {
        user_pages: new_page_associated_state.user_pages,
        journal_pages: new_page_associated_state.journal_pages,
        todo_index: TodoIndex {
            entries: todo_index_entries,
        },
        tag_index: TagIndex {
            entries: tag_index_entries,
        },
    }
}

pub fn remove_page_from_internal_state(
    file_id: &PageId,
    page_type: &PageType,
    simple_page_name: &SimplePageName,
    page_associated_state: CurrentPageAssociatedState,
) -> NewPageAssociatedState {
    let new_page_index;
    let new_journal_index;
    if let PageType::UserPage = page_type {
        new_page_index =
            remove_file_from_page_index(&page_associated_state.user_pages, &simple_page_name);
        new_journal_index = page_associated_state.journal_pages.clone();
    } else {
        new_journal_index =
            remove_file_from_journal_index(&page_associated_state.journal_pages, &simple_page_name);
        new_page_index = page_associated_state.user_pages.clone();
    }
    let new_tag_index = remove_file_from_tag_index(&page_associated_state.tag_index, &file_id);
    let new_todo_index =
        remove_file_from_todo_index(&page_associated_state.todo_index, &simple_page_name);

    NewPageAssociatedState {
        user_pages: new_page_index,
        journal_pages: new_journal_index,
        todo_index: new_todo_index,
        tag_index: new_tag_index,
    }
}
