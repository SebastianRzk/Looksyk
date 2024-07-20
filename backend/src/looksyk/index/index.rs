use crate::looksyk::index::tag::{create_tag_index_file, remove_file_from_tag_index};
use crate::looksyk::index::todo::{create_todo_index_file, remove_file_from_todo_index};
use crate::looksyk::index::userpage::{remove_file_from_journal_index, remove_file_from_page_index};
use crate::looksyk::model::{PageId, PageType, ParsedMarkdownFile, SimplePageName};
use crate::state::{JournalPageIndex, TagIndex, TodoIndex, UserPageIndex};

pub fn update_index_for_file(file_id: PageId,
                             simple_page_name: &SimplePageName,
                             page_type: &PageType,
                             update: &ParsedMarkdownFile,
                             todo_index: &TodoIndex,
                             tag_index: &TagIndex,
                             user_page_index: &UserPageIndex,
                             journal_page_index: &JournalPageIndex,
) -> (TodoIndex, TagIndex, UserPageIndex, JournalPageIndex) {
    let new_todo_index = remove_file_from_todo_index(&todo_index, &simple_page_name);
    let new_tag_index = remove_file_from_tag_index(&tag_index, &file_id);
    let mut new_page_index = remove_file_from_page_index(&user_page_index, &simple_page_name);
    let mut new_journal_index = remove_file_from_journal_index(&journal_page_index, &simple_page_name);



    let mut todo_index_entries = new_todo_index.entries;
    create_todo_index_file(&mut todo_index_entries, &file_id, &simple_page_name, update);

    let mut tag_index_entries = new_tag_index.entries;
    create_tag_index_file(&mut tag_index_entries, &file_id, &update);

    match page_type {
        PageType::UserPage => {
            new_page_index.entries.insert(simple_page_name.clone(), update.clone());
        }
        PageType::JournalPage => {
            new_journal_index.entries.insert(simple_page_name.clone(), update.clone());
        }
    }


    return (TodoIndex {
        entries: todo_index_entries
    }, TagIndex {
        entries: tag_index_entries,
    }, new_page_index, new_journal_index);
}

