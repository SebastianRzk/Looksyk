use crate::looksyk::index::hierachy::HierarchyParser;
use crate::looksyk::model::{BlockTokenType, PageId, ParsedMarkdownFile, SimplePageName};
use crate::state::journal::JournalPageIndex;
use crate::state::todo::{TodoIndex, TodoIndexEntry, TodoSourceReference, TodoState};
use crate::state::userpage::UserPageIndex;

pub fn create_todo_index(
    data_state: &UserPageIndex,
    journal_state: &JournalPageIndex,
) -> TodoIndex {
    let mut result = vec![];

    for simple_page_name in data_state.entries.keys() {
        if let Some(file) = (&data_state.entries).get(simple_page_name) {
            create_todo_index_file(&mut (&mut result), &simple_page_name.as_user_page(), file);
        }
    }
    for simple_page_name in journal_state.entries.keys() {
        if let Some(file) = &journal_state.entries.get(simple_page_name) {
            create_todo_index_file(
                &mut (&mut result),
                &simple_page_name.as_journal_page(),
                file,
            );
        }
    }

    TodoIndex { entries: result }
}

pub fn create_todo_index_file(
    result: &mut Vec<TodoIndexEntry>,
    page_id: &PageId,
    file: &ParsedMarkdownFile,
) {
    let mut hierarchy_index = HierarchyParser {
        page_name: page_id.name.clone(),
        current_hierarchy: vec![],
    };

    let mut blocknumber: usize = 0;
    for block in &file.blocks {
        hierarchy_index.feed(block);
        if block.content.len() > 0 {
            let content_line = block.content.get(0).unwrap();
            if content_line.as_tokens.len() > 0 {
                let first_token = content_line.as_tokens.get(0).unwrap();
                if first_token.block_token_type == BlockTokenType::TODO {
                    result.push(TodoIndexEntry {
                        block: block.clone(),
                        source: TodoSourceReference {
                            page_id: page_id.clone(),
                            blocknumber,
                        },
                        state: state_from_payload(&first_token.payload),
                        tags: hierarchy_index.get_current_tag_set(),
                    })
                }
            }
        }
        blocknumber += 1;
    }
}

pub fn remove_file_from_todo_index(todo_index: &TodoIndex, tag_name: &SimplePageName) -> TodoIndex {
    let mut result = vec![];
    for entry in &todo_index.entries {
        if entry.source.page_id.name.name != tag_name.name {
            result.push(entry.clone());
        }
    }
    TodoIndex { entries: result }
}

fn state_from_payload(payload: &String) -> TodoState {
    if payload.eq(&" ".to_string()) {
        return TodoState::Todo;
    }
    TodoState::Done
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::builder::{
        any_text_token, done_token, empty_journal_index, todo_token, user_page_id,
    };
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::index::todo::create_todo_index;
    use crate::looksyk::model::{BlockContent, ParsedBlock, ParsedMarkdownFile};
    use crate::state::todo::{TodoSourceReference, TodoState};
    use crate::state::userpage::UserPageIndex;
    use std::collections::HashMap;

    #[test]
    pub fn non_todo_file_should_return_empty_index() {
        let mut data_state = HashMap::new();
        data_state.insert(
            page_name_str("testfile"),
            ParsedMarkdownFile {
                blocks: vec![ParsedBlock {
                    indentation: 0,
                    content: vec![],
                }],
            },
        );

        let result = create_todo_index(
            &UserPageIndex {
                entries: data_state,
            },
            &empty_journal_index(),
        );

        assert_eq!(result.entries.len(), 0);
    }

    #[test]
    pub fn todo_without_tags_should_insert_index_entry() {
        let mut data_state = HashMap::new();
        data_state.insert(
            page_name_str("testfile"),
            ParsedMarkdownFile {
                blocks: vec![ParsedBlock {
                    indentation: 0,
                    content: vec![BlockContent {
                        as_tokens: vec![todo_token(), any_text_token()],
                        as_text: "".to_string(),
                    }],
                }],
            },
        );

        let result = create_todo_index(
            &UserPageIndex {
                entries: data_state,
            },
            &empty_journal_index(),
        );

        assert_eq!(result.entries.len(), 1);
        let entry = result.entries.get(0).unwrap();
        assert_eq!(entry.tags, vec![page_name_str("testfile")]);
        assert_eq!(entry.state, TodoState::Todo);
        assert_eq!(
            entry.source,
            TodoSourceReference {
                page_id: user_page_id("testfile"),
                blocknumber: 0,
            }
        )
    }

    #[test]
    pub fn todo_done_without_tags_should_insert_index_entry() {
        let mut data_state = HashMap::new();
        data_state.insert(
            page_name_str("testfile"),
            ParsedMarkdownFile {
                blocks: vec![ParsedBlock {
                    indentation: 0,
                    content: vec![BlockContent {
                        as_tokens: vec![done_token(), any_text_token()],
                        as_text: "".to_string(),
                    }],
                }],
            },
        );

        let result = create_todo_index(
            &UserPageIndex {
                entries: data_state,
            },
            &empty_journal_index(),
        );

        assert_eq!(result.entries.len(), 1);
        let entry = result.entries.get(0).unwrap();
        assert_eq!(entry.tags, vec![page_name_str("testfile")]);
        assert_eq!(entry.state, TodoState::Done);
        assert_eq!(
            entry.source,
            TodoSourceReference {
                page_id: user_page_id("testfile"),
                blocknumber: 0,
            }
        )
    }
}
