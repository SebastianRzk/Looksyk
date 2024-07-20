use std::collections::{HashMap, HashSet};

use crate::looksyk::builder::page_name;
use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, PageId, PageType, ParsedBlock, ParsedMarkdownFile};
use crate::looksyk::page_index::{append_journal_page_prefix, append_user_page_prefix, get_page_type, strip_journal_page_prefix, strip_user_page_prefix};
use crate::state::{JournalPageIndex, TagIndex, UserPageIndex};

pub fn create_tag_index(data_state: &UserPageIndex, journal_page_index: &JournalPageIndex) -> TagIndex {
    let mut result: HashMap<PageId, HashSet<PageId>> = HashMap::new();

    for simple_page_name in data_state.entries.keys() {
        let page = data_state.entries.get(simple_page_name).unwrap();
        let id = append_user_page_prefix(simple_page_name);
        create_tag_index_file(&mut result, &id, &page);
    }

    for simple_page_name in journal_page_index.entries.keys() {
        let page = journal_page_index.entries.get(simple_page_name).unwrap();
        let id = append_journal_page_prefix(simple_page_name);
        create_tag_index_file(&mut result, &id, &page);
    }

    TagIndex {
        entries: result
    }
}

pub fn create_tag_index_file(result: &mut HashMap<PageId, HashSet<PageId>>, current_page_name: &PageId, file: &&ParsedMarkdownFile) {
    for block in &file.blocks {
        for content in &block.content {
            for token in &content.as_tokens {
                if token.block_token_type != BlockTokenType::LINK {
                    continue;
                }
                let payload = token.payload.clone();
                let encoded_payload = payload.replace("/", "%2F");
                let tag_name = append_user_page_prefix(&page_name(encoded_payload));
                let stored_list = result.get(&tag_name);
                let mut tokenlist;
                if stored_list.is_none() {
                    tokenlist = HashSet::new();
                } else {
                    tokenlist = stored_list.unwrap().clone();
                }

                tokenlist.insert(current_page_name.clone());
                result.insert(tag_name, tokenlist);
            }
        }
    }
}

pub fn remove_file_from_tag_index(tag_index: &TagIndex, file_id: &PageId) -> TagIndex {
    println!("Removing file {} from tag index", file_id.id);
    let page_type = get_page_type(file_id);
    if page_type == PageType::JournalPage {
        return tag_index.clone();
    }

    let mut result = HashMap::new();
    for key in tag_index.entries.keys() {
        if key != file_id {
            let current_list = tag_index.entries.get(key);
            let referenced_tags = filter_tag(current_list.unwrap(), &file_id);
            result.insert(key.clone(), referenced_tags);
        }
    }
    TagIndex {
        entries: result
    }
}

pub fn render_tag_index_for_page(page_id: PageId, tag_index: &TagIndex) -> ParsedMarkdownFile {
    let empty_set = HashSet::new();
    let tags_for_page = tag_index.entries.get(&page_id).or_else(|| {
        Some(&empty_set)
    }).unwrap();

    if tags_for_page.is_empty() {
        return ParsedMarkdownFile {
            blocks: vec![no_references_found_text(0)]
        };
    }

    let mut sorted_pages = tags_for_page.clone().into_iter().collect::<Vec<PageId>>();
    sorted_pages.sort_by(|a, b| a.id.cmp(&b.id));
    let page_references = sorted_pages.iter().filter(|p| get_page_type(p) == PageType::UserPage).collect::<Vec<&PageId>>();
    let journal_references = sorted_pages.iter().filter(|p| get_page_type(p) == PageType::JournalPage).collect::<Vec<&PageId>>();

    let mut blocks: Vec<ParsedBlock> = vec![];
    blocks.append(&mut reference_entry_group(&page_references, "Wiki-Pages"));
    blocks.append(&mut reference_entry_group(&journal_references, "Journal-Pages"));

    return ParsedMarkdownFile {
        blocks
    };
}

fn reference_entry_group(page_references: &Vec<&PageId>, name: &str) -> Vec<ParsedBlock> {
    let mut blocks = vec![ParsedBlock {
        indentation: 0,
        content: vec![BlockContent {
            as_tokens: vec![BlockToken {
                payload: format!("{} that reference this page", name),
                block_token_type: BlockTokenType::TEXT,
            }],
            as_text: format!("{} that reference this page", name),
        }],
    }];

    for tag in page_references {
        let page_type = get_page_type(tag);
        match page_type {
            PageType::JournalPage => blocks.push(ParsedBlock {
                indentation: 1,
                content: vec![BlockContent {
                    as_tokens: vec![BlockToken {
                        payload: strip_journal_page_prefix(tag).name.clone(),
                        block_token_type: BlockTokenType::JOURNAL_LINK,
                    }],
                    as_text: strip_user_page_prefix(tag).name.clone(),
                }],
            })
            ,
            PageType::UserPage => blocks.push(ParsedBlock {
                indentation: 1,
                content: vec![BlockContent {
                    as_tokens: vec![BlockToken {
                        payload: strip_user_page_prefix(tag).name.clone(),
                        block_token_type: BlockTokenType::LINK,
                    }],
                    as_text: strip_user_page_prefix(tag).name.clone(),
                }],
            })
        }
    }
    if page_references.is_empty() {
        blocks.push(no_references_found_text(1));
    }
    blocks
}

fn no_references_found_text(indentation: usize) -> ParsedBlock {
    ParsedBlock {
        indentation: indentation,
        content: vec![BlockContent {
            as_tokens: vec![BlockToken {
                payload: "No references found".to_string(),
                block_token_type: BlockTokenType::TEXT,
            }],
            as_text: "No references found".to_string(),
        }],
    }
}

fn filter_tag(current_list: &HashSet<PageId>, page_to_remove: &PageId) -> HashSet<PageId> {
    let mut new_list = current_list.clone();
    new_list.remove(page_to_remove);
    return new_list;
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::looksyk::builder::{any_text_token, done_token, journal_page_id, page_name_str, user_page_id};
    use crate::looksyk::index::tag::create_tag_index;
    use crate::looksyk::index::todo::create_todo_index;
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, PageId, ParsedBlock, ParsedMarkdownFile};
    use crate::state::{JournalPageIndex, UserPageIndex};

    #[test]
    pub fn should_create_tag_index_with_empty_state() {
        let data_state = HashMap::new();
        let result = create_tag_index(&UserPageIndex {
            entries: data_state
        }, &JournalPageIndex {
            entries: HashMap::new()
        });

        assert_eq!(result.entries.len(), 0);
    }


    #[test]
    pub fn should_create_tag_index_with_tag() {
        let mut data_state = HashMap::new();
        data_state.insert(page_name_str("source-page"), ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_tokens: vec![BlockToken {
                        payload: "target-page".to_string(),
                        block_token_type: BlockTokenType::LINK,
                    }],
                    as_text: "".to_string(),
                }],
            }]
        });
        let result = create_tag_index(&UserPageIndex {
            entries: data_state
        }, &empty_journal_index());

        assert_eq!(result.entries.len(), 1);
        let entry = result.entries.get(&user_page_id("target-page")).unwrap();
        assert_eq!(entry, &vec![user_page_id("source-page")].into_iter().collect::<HashSet<PageId>>());
    }

    fn empty_journal_index() -> JournalPageIndex {
        JournalPageIndex {
            entries: HashMap::new()
        }
    }

    #[test]
    pub fn should_create_tag_index_with_journal_tag() {
        let mut data_state = HashMap::new();
        data_state.insert(page_name_str("source-page"), ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_tokens: vec![BlockToken {
                        payload: "target-page".to_string(),
                        block_token_type: BlockTokenType::LINK,
                    }],
                    as_text: "".to_string(),
                }],
            }]
        });
        let result = create_tag_index(&UserPageIndex {
            entries: HashMap::new()
        }, &JournalPageIndex {
            entries: data_state
        });

        assert_eq!(result.entries.len(), 1);
        let entry = result.entries.get(&user_page_id("target-page")).unwrap();

        assert_eq!(entry, &vec![journal_page_id("source-page")].into_iter().collect::<HashSet<PageId>>());
    }


    #[test]
    pub fn render_tag_index_for_page_with_no_tags_should_render_empty_page() {
        let tag_index = create_tag_index(&UserPageIndex {
            entries: HashMap::new()
        }, &empty_journal_index());

        let result = super::render_tag_index_for_page(user_page_id("testpage"), &tag_index);

        assert_eq!(result.blocks.len(), 1);
        let block = result.blocks.get(0).unwrap();
        assert_eq!(block.content.len(), 1);
        let content = block.content.get(0).unwrap();
        assert_eq!(content.as_text, "No references found");
    }

    #[test]
    pub fn with_tags_in_line_should_insert_index_entry() {
        let mut data_state = HashMap::new();
        data_state.insert(page_name_str("testfile"), ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_tokens: vec![done_token(), any_text_token(), BlockToken {
                        payload: "MyTag".to_string(),
                        block_token_type: BlockTokenType::LINK,
                    }],
                    as_text: "".to_string(),
                }],
            }]
        }, );

        let result = create_todo_index(&UserPageIndex {
            entries: data_state,
        });

        assert_eq!(result.entries.len(), 1);
        let entry = result.entries.get(0).unwrap();
        assert_eq!(entry.tags, vec![page_name_str("testfile"), page_name_str("MyTag")]);
    }

}