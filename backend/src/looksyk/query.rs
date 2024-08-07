use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Error;

use crate::looksyk::model::{BlockToken, QueryRenderResult};
use crate::looksyk::queries::insert_file_content::{parse_query_insert_file_content, QUERY_NAME_INSERT_FILE_CONTENT, render_query_insert_file_content};
use crate::looksyk::queries::pagehierarchy::{parse_query_page_hierarchy, QUERY_NAME_PAGE_HIERARCHY, render_page_hierarchy};
use crate::looksyk::queries::references_to::{parse_query_references_to, QUERY_NAME_REFERENCES_TO, render_references_of_query};
use crate::looksyk::queries::todo::{parse_query_todo, QUERY_NAME_TODOS, render_todo_query};
use crate::state::asset_cache::AssetCache;
use crate::state::state::DataRootLocation;
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;
use crate::state::userpage::UserPageIndex;

pub fn render_query(
    block: &BlockToken,
    data: &UserPageIndex,
    todo_index: &TodoIndex,
    tag_index: &TagIndex,
    asset_cache: &mut AssetCache,
    data_root_location: &DataRootLocation,
) -> QueryRenderResult {
    let query = parse_query(&block.payload);
    if query.is_err() {
        let error = query.err().unwrap();
        println!("Error on parsing query {}", &error);
        return QueryRenderResult {
            inplace_markdown: format!("\n\nError on parsing query: {}\n\n", error.to_string()),
            referenced_markdown: vec![],
        };
    }
    render_parsed_query(query.unwrap(), data, todo_index, tag_index, asset_cache, data_root_location)
}


pub fn parse_query(payload: &String) -> Result<Query, Error> {
    let query_str = payload.trim();
    if query_str.starts_with(QUERY_NAME_PAGE_HIERARCHY) {
        return parse_query_page_hierarchy(query_str);
    } else if query_str.starts_with(QUERY_NAME_TODOS) {
        return parse_query_todo(query_str);
    } else if query_str.starts_with(QUERY_NAME_REFERENCES_TO) {
        return parse_query_references_to(query_str);
    } else if query_str.starts_with(QUERY_NAME_INSERT_FILE_CONTENT) {
        return parse_query_insert_file_content(query_str);
    }
    Ok(Query {
        query_type: QueryType::Unknown,
        display: QueryDisplayType::Unknown,
        args: HashMap::new(),
    })
}


pub fn render_parsed_query(query: Query, data: &UserPageIndex, todo_index: &TodoIndex, tag_index: &TagIndex, asset_cache: &mut AssetCache, data_root_location: &DataRootLocation) -> QueryRenderResult {
    match query.query_type {
        QueryType::PageHierarchy => {
            render_page_hierarchy(query, &data)
        }
        QueryType::Todo => {
            render_todo_query(query, todo_index)
        }
        QueryType::ReferencesTo => {
            render_references_of_query(query, tag_index)
        }
        QueryType::InsertFileContent => {
            render_query_insert_file_content(query, asset_cache, data_root_location)
        }
        QueryType::Unknown => {
            QueryRenderResult {
                inplace_markdown: "Query type unknown".to_string(),
                referenced_markdown: vec![],
            }
        }
    }
}


pub struct Query {
    pub query_type: QueryType,
    pub args: HashMap<String, String>,
    pub display: QueryDisplayType,
}

#[derive(PartialEq, Debug)]
pub enum QueryType {
    PageHierarchy,
    ReferencesTo,
    Todo,
    InsertFileContent,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum QueryDisplayType {
    ReferencedList,
    InplaceList,
    CodeBlock,
    InlineText,
    Count,
    Unknown,
}

impl Display for QueryDisplayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryDisplayType::ReferencedList => write!(f, "referenced-list"),
            QueryDisplayType::InplaceList => write!(f, "inplace-list"),
            QueryDisplayType::Count => write!(f, "count"),
            QueryDisplayType::Unknown => write!(f, "unknown"),
            QueryDisplayType::CodeBlock => write!(f, "code-block"),
            QueryDisplayType::InlineText => write!(f, "inline-text"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::path::Path;

    use crate::io::fs::media::MediaOnDisk;
    use crate::looksyk::builder::{page_id, page_name_str, text_token, user_page_id};
    use crate::looksyk::index::asset::create_empty_asset_cache;
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, PageId, PageType, ParsedBlock, ParsedMarkdownFile};
    use crate::looksyk::query::{parse_query, QueryDisplayType, QueryType, render_query};
    use crate::state::asset_cache::{AssetFileContent, AssetState, FileSizeViolation};
    use crate::state::state::DataRootLocation;
    use crate::state::tag::TagIndex;
    use crate::state::todo::{TodoIndex, TodoIndexEntry, TodoSourceReference, TodoState};
    use crate::state::userpage::UserPageIndex;

    #[test]
    pub fn should_render_unknown_query_on_unknown_query() {
        let result = render_query(
            &BlockToken {
                block_token_type: BlockTokenType::QUERY,
                payload: "unknown asdf".to_string(),
            },
            &empty_page_index(),
            &empty_todo_index(),
            &empty_tag_index(),
            &mut create_empty_asset_cache(),
            &empty_root_location(),
        );
        assert_eq!(result.inplace_markdown, "Query type unknown");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    fn empty_todo_index() -> TodoIndex {
        TodoIndex {
            entries: vec![]
        }
    }

    fn empty_tag_index() -> TagIndex {
        TagIndex {
            entries: HashMap::new()
        }
    }

    #[test]
    pub fn should_render_hierarchy_query_as_list() {
        let mut hierarchy_data = HashMap::new();
        hierarchy_data.insert(page_name_str("parent / sub1"), empty_file());
        hierarchy_data.insert(page_name_str("parent / sub2"), empty_file());
        hierarchy_data.insert(page_name_str("other / unknown"), empty_file());

        let result = render_query(&BlockToken {
            payload: "page-hierarchy root:\"parent\" display:\"inplace-list\" ".to_string(),
            block_token_type: BlockTokenType::QUERY,
        },
                                  &UserPageIndex {
                                      entries: hierarchy_data,
                                  }, &empty_todo_index(), &empty_tag_index(),
                                  &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "parent:\n- [parent / sub1](page/parent%20%2F%20sub1)\n- [parent / sub2](page/parent%20%2F%20sub2)\n");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    fn empty_root_location() -> DataRootLocation {
        DataRootLocation {
            name: "".to_string(),
            path: Path::new("").to_path_buf(),
        }
    }

    #[test]
    pub fn should_render_hierarchy_query_with_unknown_displaytype() {
        let mut hierarchy_data = HashMap::new();
        hierarchy_data.insert(page_name_str("parent / sub1"), empty_file());
        hierarchy_data.insert(page_name_str("parent / sub2"), empty_file());
        hierarchy_data.insert(page_name_str("other / unknown"), empty_file());

        let result = render_query(
            &BlockToken {
                payload: "page-hierarchy root:\"parent\" display:\"unknown\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            },
            &UserPageIndex {
                entries: hierarchy_data
            }, &empty_todo_index(), &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "Decode error: Unknown display type");
        assert_eq!(result.referenced_markdown.len(), 0);
    }


    #[test]
    pub fn should_render_hierarchy_with_unsupported_displaytype() {
        let mut hierarchy_data = HashMap::new();
        hierarchy_data.insert(page_name_str("parent / sub1"), empty_file());
        hierarchy_data.insert(page_name_str("parent / sub2"), empty_file());
        hierarchy_data.insert(page_name_str("other / unknown"), empty_file());

        let result = render_query(
            &BlockToken {
                payload: "page-hierarchy root:\"parent\" display:\"referenced-list\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            },
            &UserPageIndex {
                entries: hierarchy_data
            }, &empty_todo_index(), &empty_tag_index(),
            &mut create_empty_asset_cache(),
            &empty_root_location());

        assert_eq!(result.inplace_markdown, "display type referenced-list not suppoerted for querytype");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_parse_query() {
        let result = parse_query(&"page-hierarchy root:\"parent\" display:\"inplace-list\" ".to_string()).unwrap();

        assert_eq!(result.query_type, QueryType::PageHierarchy);
        assert_eq!(result.display, QueryDisplayType::InplaceList);
        assert_eq!(result.args.keys().len(), 1);
        assert_eq!(result.args.get(&"root".to_string()).unwrap(), "parent");
    }

    #[test]
    pub fn should_catch_error_on_parsing_error() {
        let result = parse_query(&"page-hierarchy asdf:\"asd\" display:\"\"\" ".to_string());
        assert_eq!(result.is_err(), true);
    }


    #[test]
    pub fn should_render_page_hierarchy_as_count() {
        let mut hierarchy_data = HashMap::new();
        hierarchy_data.insert(page_name_str("parent / sub1"), empty_file());
        hierarchy_data.insert(page_name_str("parent / sub2"), empty_file());
        hierarchy_data.insert(page_name_str("other / unknown"), empty_file());

        let result = render_query(
            &BlockToken {
                payload: "page-hierarchy root:\"parent\" display:\"count\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            },
            &UserPageIndex {
                entries: hierarchy_data
            }, &empty_todo_index(), &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "2");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    fn empty_file() -> ParsedMarkdownFile {
        ParsedMarkdownFile {
            blocks: vec![]
        }
    }

    #[test]
    pub fn should_render_todo_as_count() {
        let todo_index = TodoIndex {
            entries: vec![TodoIndexEntry {
                block: ParsedBlock {
                    indentation: 0,
                    content: vec![BlockContent {
                        as_tokens: vec![],
                        as_text: "".to_string(),
                    }],
                },
                tags: vec![page_name_str("parent")],
                state: TodoState::Todo,
                source: TodoSourceReference {
                    page_id: page_id("%%user/testfile"),
                    page_name: page_name_str("testfile"),
                    blocknumber: 0,
                    page_type: PageType::UserPage,
                },
            }]
        };

        let result = render_query(
            &BlockToken {
                payload: "todos tag:\"parent\" state:\"todo\" display:\"count\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &todo_index, &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "1");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_todo_as_list() {
        let todo_index = TodoIndex {
            entries: vec![
                TodoIndexEntry {
                    block: ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_tokens: vec![text_token("[ ] todo not done")],
                            as_text: "[ ] todo not done".to_string(),
                        }],
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Todo,
                    source: TodoSourceReference {
                        page_id: page_id("%%user/testfile"),
                        page_name: page_name_str("testfile"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                },
                TodoIndexEntry {
                    block: ParsedBlock {
                        content: vec![BlockContent {
                            as_tokens: vec![text_token("[x] todo done")],
                            as_text: "[x] todo done".to_string(),
                        }],
                        indentation: 0,
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Done,
                    source: TodoSourceReference {
                        page_id: page_id("%%user/testfile"),
                        page_name: page_name_str("testfile2"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                }]
        };
        let mut entries = HashMap::new();
        entries.insert(page_name_str("testfile"), empty_file());
        let page_index = UserPageIndex {
            entries
        };

        let result = render_query(
            &BlockToken {
                payload: "todos tag:\"parent\" state:\"todo\" display:\"inplace-list\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &page_index, &todo_index, &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "\n\n* :white large square: [testfile](page/testfile): todo not done\n\n".to_string());
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_todo_done_as_list() {
        let todo_index = TodoIndex {
            entries: vec![
                TodoIndexEntry {
                    block: ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_tokens: vec![text_token("[ ] todo not done")],
                            as_text: "[ ] todo not done".to_string(),
                        }],
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Todo,
                    source: TodoSourceReference {
                        page_id: page_id("%%user/testfile"),
                        page_name: page_name_str("testfile"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                },
                TodoIndexEntry {
                    block: ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_tokens: vec![text_token("[x] todo done")],
                            as_text: "[x] todo done".to_string(),
                        }],
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Done,
                    source: TodoSourceReference {
                        page_id: page_id("%%user/testfile2"),
                        page_name: page_name_str("testfile2"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                }]
        };
        let mut entries = HashMap::new();
        entries.insert(page_name_str("testfile"), empty_file());
        let page_index = UserPageIndex {
            entries
        };

        let result = render_query(
            &BlockToken {
                payload: "todos tag:\"parent\" state:\"done\" display:\"inplace-list\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &page_index, &todo_index, &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "\n\n* :check mark: [testfile2](page/testfile2): todo done\n\n".to_string());
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    fn empty_page_index() -> UserPageIndex {
        UserPageIndex {
            entries: HashMap::new()
        }
    }


    #[test]
    pub fn should_render_todo_done_as_reference_list() {
        let todo_index = TodoIndex {
            entries: vec![
                TodoIndexEntry {
                    block: ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_tokens: vec![],
                            as_text: "todo not done".to_string(),
                        }],
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Todo,
                    source: TodoSourceReference {
                        page_name: page_name_str("testfile"),
                        page_id: page_id("%%user/testfile"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                },
                TodoIndexEntry {
                    block: ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_tokens: vec![],
                            as_text: "todo done".to_string(),
                        }],
                    },
                    tags: vec![page_name_str("parent")],
                    state: TodoState::Done,
                    source: TodoSourceReference {
                        page_name: page_name_str("testfile2"),
                        page_id: page_id("%%user/testfile2"),
                        blocknumber: 0,
                        page_type: PageType::UserPage,
                    },
                }]
        };

        let result = render_query(
            &BlockToken {
                payload: "todos tag:\"parent\" state:\"done\" display:\"referenced-list\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &todo_index, &empty_tag_index(),
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "");
        assert_eq!(result.referenced_markdown.len(), 1);
        let reference = result.referenced_markdown.get(0).unwrap();
        assert_eq!(reference.refernce.page_name.name, "testfile2");
        assert_eq!(reference.refernce.block_number, 0);
        assert_eq!(reference.content.content.len(), 1);
        assert_eq!(reference.content.content.get(0).unwrap().as_tokens, vec![]);
        assert_eq!(reference.content.content.get(0).unwrap().as_text, "todo done");
    }

    #[test]
    pub fn should_render_references_to_as_count() {
        let mut tag_index_entries: HashMap<PageId, HashSet<PageId>> = HashMap::new();
        let mut references: HashSet<PageId> = HashSet::new();
        references.insert(user_page_id("something"));
        tag_index_entries.insert(user_page_id("mysite123"), references);
        let tag_index = TagIndex {
            entries: tag_index_entries
        };

        let result = render_query(
            &BlockToken {
                payload: "references-to target:\"mysite123\" display:\"count\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &tag_index,
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "1");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_references_to_as_inplace_list() {
        let mut tag_index_entries: HashMap<PageId, HashSet<PageId>> = HashMap::new();
        let mut references: HashSet<PageId> = HashSet::new();
        references.insert(user_page_id("something"));
        tag_index_entries.insert(user_page_id("mysite123"), references);
        let tag_index = TagIndex {
            entries: tag_index_entries
        };

        let result = render_query(
            &BlockToken {
                payload: "references-to target:\"mysite123\" display:\"inplace-list\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &tag_index,
            &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "Pages that reference [mysite123](page/mysite123)\n* [something](page/something)\n");
        assert_eq!(result.referenced_markdown.len(), 0);
    }


    #[test]
    pub fn should_render_references_to_as_inplace_list_no_ref_should_show_no_ref_message() {
        let result = render_query(&BlockToken {
            payload: "references-to target:\"mysite123\" display:\"inplace-list\" ".to_string(),
            block_token_type: BlockTokenType::QUERY,
        }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
                                  &mut create_empty_asset_cache(), &empty_root_location());

        assert_eq!(result.inplace_markdown, "Pages that reference [mysite123](page/mysite123)\n* No references found!\n");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_inline_text_with_inline_text() {
        let mut asset_cache = create_empty_asset_cache();
        asset_cache.insert(
            &MediaOnDisk {
                name: "myfile".to_string(),
            },
            AssetState::Found(AssetFileContent {
                content: "myFileContent".to_string()
            }),
        );

        let result = render_query(
            &BlockToken {
                payload: "insert-file-content target-file:\"myfile\" display:\"inline-text\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
            &mut asset_cache, &empty_root_location());

        assert_eq!(result.inplace_markdown, "myFileContent");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_inline_text_with_file_not_found() {
        let mut asset_cache = create_empty_asset_cache();
        asset_cache.insert(
            &MediaOnDisk {
                name: "myfile".to_string(),
            },
            AssetState::NotFound,
        );

        let result = render_query(
            &BlockToken {
                payload: "insert-file-content target-file:\"myfile\" display:\"inline-text\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
            &mut asset_cache, &empty_root_location());

        assert_eq!(result.inplace_markdown, "File not found");
        assert_eq!(result.referenced_markdown.len(), 0);
    }


    #[test]
    pub fn should_render_inline_text_with_file_too_large() {
        let mut asset_cache = create_empty_asset_cache();
        asset_cache.insert(
            &MediaOnDisk {
                name: "myfile".to_string(),
            },
            AssetState::TooLarge(FileSizeViolation{
                file_size: 1025,
                max_size: 512
            }),
        );

        let result = render_query(
            &BlockToken {
                payload: "insert-file-content target-file:\"myfile\" display:\"inline-text\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
            &mut asset_cache, &empty_root_location());

        assert_eq!(result.inplace_markdown, "File is too large. Max size is 512. File size is 1025");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_inline_text_with_file_not_text() {
        let mut asset_cache = create_empty_asset_cache();
        asset_cache.insert(
            &MediaOnDisk {
                name: "myfile".to_string(),
            },
            AssetState::NotText
        );

        let result = render_query(
            &BlockToken {
                payload: "insert-file-content target-file:\"myfile\" display:\"inline-text\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
            &mut asset_cache, &empty_root_location());

        assert_eq!(result.inplace_markdown, "File is not a text file. Can not inline a binary file");
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    pub fn should_render_inline_text_as_code() {
        let mut asset_cache = create_empty_asset_cache();
        asset_cache.insert(
            &MediaOnDisk {
                name: "myfile.rs".to_string(),
            },
            AssetState::Found(AssetFileContent {
                content: "myFileContent".to_string()
            }),
        );

        let result = render_query(
            &BlockToken {
                payload: "insert-file-content target-file:\"myfile.rs\" display:\"code-block\" ".to_string(),
                block_token_type: BlockTokenType::QUERY,
            }, &empty_page_index(), &empty_todo_index(), &empty_tag_index(),
            &mut asset_cache, &empty_root_location());

        assert_eq!(result.inplace_markdown, "```rust\nmyFileContent\n```");
        assert_eq!(result.referenced_markdown.len(), 0);
    }
}