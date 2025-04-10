use crate::looksyk::index::tag::{create_tag_index_file, remove_file_from_tag_index};
use crate::looksyk::index::todo::{create_todo_index_file, remove_file_from_todo_index};
use crate::looksyk::index::userpage::{
    remove_file_from_journal_index, remove_file_from_page_index,
};
use crate::looksyk::model::{PageId, PageType, ParsedMarkdownFile};
use crate::state::application_state::{CurrentPageAssociatedState, NewPageAssociatedState};
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;

pub fn update_index_for_file(
    page_id: PageId,
    update: &ParsedMarkdownFile,
    page_associated_state: CurrentPageAssociatedState,
) -> NewPageAssociatedState {
    let mut new_page_associated_state =
        remove_page_from_internal_state(&page_id, page_associated_state);

    let mut todo_index_entries = new_page_associated_state.todo_index.entries;
    create_todo_index_file(&mut todo_index_entries, &page_id, update);

    let mut tag_index_entries = new_page_associated_state.tag_index.entries;
    create_tag_index_file(&mut tag_index_entries, &page_id, update);

    match page_id.page_type {
        PageType::UserPage => {
            new_page_associated_state
                .user_pages
                .entries
                .insert(page_id.name.clone(), update.clone());
        }
        PageType::JournalPage => {
            new_page_associated_state
                .journal_pages
                .entries
                .insert(page_id.name.clone(), update.clone());
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
    page_id: &PageId,
    page_associated_state: CurrentPageAssociatedState,
) -> NewPageAssociatedState {
    let new_page_index;
    let new_journal_index;
    if let PageType::UserPage = page_id.page_type {
        new_page_index =
            remove_file_from_page_index(page_associated_state.user_pages, &page_id.name);
        new_journal_index = page_associated_state.journal_pages.clone();
    } else {
        new_journal_index =
            remove_file_from_journal_index(page_associated_state.journal_pages, &page_id.name);
        new_page_index = page_associated_state.user_pages.clone();
    }
    let new_tag_index = remove_file_from_tag_index(page_associated_state.tag_index, page_id);
    let new_todo_index =
        remove_file_from_todo_index(page_associated_state.todo_index, &page_id.name);

    NewPageAssociatedState {
        user_pages: new_page_index,
        journal_pages: new_journal_index,
        todo_index: new_todo_index,
        tag_index: new_tag_index,
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::looksyk::index::index_operations::update_index_for_file;
    use crate::looksyk::model::{ParsedMarkdownFile, RawBlock};
    use crate::looksyk::parser::parse_block;
    use crate::state::application_state::CurrentPageAssociatedState;
    use crate::state::journal::JournalPageIndex;
    use crate::state::tag::TagIndex;
    use crate::state::todo::TodoIndex;
    use crate::state::userpage::UserPageIndex;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_update_index_for_file_should_refresh_tag_index() {
        let page_id = user_page_id("test-page-name");
        let parsed_markdown_file = ParsedMarkdownFile {
            blocks: vec![parse_block(&RawBlock {
                indentation: 0,
                text_content: vec!["[[new-link-on-page]]".to_string()],
            })],
        };

        let mut tag_index_map = HashMap::new();
        let mut test_page_backlinks = HashSet::new();
        test_page_backlinks.insert(user_page_id("other-page-name"));
        let mut other_page_backlinks = HashSet::new();
        other_page_backlinks.insert(user_page_id("test-page-name"));

        tag_index_map.insert(user_page_id("test-page-name"), test_page_backlinks);
        tag_index_map.insert(user_page_id("other-page-name"), other_page_backlinks);
        let tag_index = TagIndex {
            entries: tag_index_map,
        };
        let current_page_associated_state = CurrentPageAssociatedState {
            tag_index: &tag_index,
            todo_index: &TodoIndex { entries: vec![] },
            user_pages: &UserPageIndex {
                entries: HashMap::new(),
            },
            journal_pages: &JournalPageIndex {
                entries: HashMap::new(),
            },
        };

        let result = update_index_for_file(
            page_id.clone(),
            &parsed_markdown_file,
            current_page_associated_state,
        );

        assert_eq!(result.tag_index.entries.get(&page_id).unwrap().len(), 1);
        assert!(result
            .tag_index
            .entries
            .get(&page_id)
            .unwrap()
            .contains(&user_page_id("other-page-name")));
        assert!(result
            .tag_index
            .entries
            .get(&user_page_id("new-link-on-page"))
            .unwrap()
            .contains(&page_id));
    }
}
