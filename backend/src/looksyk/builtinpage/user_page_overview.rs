use crate::looksyk::builtinpage::generating_page_util::create_textblock;
use crate::looksyk::model::{
    BlockToken, BlockTokenType, PageId, PageType, ParsedBlock, ParsedMarkdownFile, SimplePageName,
};
use crate::state::tag::TagIndex;
use crate::state::userpage::UserPageIndex;
use std::collections::HashSet;

pub fn generate_overview_page(
    all_tags: &TagIndex,
    all_pages: &UserPageIndex,
) -> ParsedMarkdownFile {
    let mut result = vec![];

    if all_tags.entries.is_empty() && all_pages.entries.is_empty() {
        result.push(create_textblock("No tags or pages found!", 0));
    } else {
        let mut result_table = vec![table_headline()];

        let mut visited_pages = HashSet::new();

        let entries = get_sorted(all_tags);
        for tag in entries {
            if tag.page_type == PageType::JournalPage {
                continue;
            }
            render_table_line(
                all_pages,
                &mut result_table,
                &tag.name,
                all_tags.entries.get(tag).unwrap(),
            );
            visited_pages.insert(tag);
        }

        let mut keys: Vec<&SimplePageName> = all_pages.entries.keys().clone().collect();
        keys.sort_by(|a, b| a.name.cmp(&b.name));
        for simple_page_name in keys {
            let id = simple_page_name.as_user_page();
            if !visited_pages.contains(&id) {
                render_table_line(
                    all_pages,
                    &mut result_table,
                    simple_page_name,
                    &HashSet::new(),
                );
            }
        }

        result.push(ParsedBlock::from_tokens(result_table))
    }

    ParsedMarkdownFile { blocks: result }
}

fn get_sorted(all_tags: &TagIndex) -> Vec<&PageId> {
    let mut entries: Vec<&PageId> = all_tags.entries.keys().clone().collect();
    entries.sort_by(|a, b| a.name.name.cmp(&b.name.name));
    entries
}

fn table_headline() -> BlockToken {
    BlockToken {
        payload: "| pagename | number of backlinks | page has content |\n| :-- | :-- | :-- |\n"
            .to_string(),
        block_token_type: BlockTokenType::Text,
    }
}

fn render_table_line(
    all_pages: &UserPageIndex,
    result_table: &mut Vec<BlockToken>,
    tag: &SimplePageName,
    references: &HashSet<PageId>,
) {
    result_table.push(BlockToken {
        block_token_type: BlockTokenType::Text,
        payload: "| ".to_string(),
    });
    result_table.push(BlockToken {
        block_token_type: BlockTokenType::Link,
        payload: tag.name.clone(),
    });
    result_table.push(BlockToken {
        block_token_type: BlockTokenType::Text,
        payload: format!(
            " | {} | {} |\n",
            references.len(),
            get_display_text_page_created(tag, all_pages)
        )
        .to_string(),
    });
}

fn get_display_text_page_created(
    simple_page_name: &SimplePageName,
    all_data: &UserPageIndex,
) -> String {
    if all_data.entries.contains_key(simple_page_name) {
        return "yes".to_string();
    }
    "not yet".to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::looksyk::builtinpage::user_page_overview::generate_overview_page;
    use crate::looksyk::model::{BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile};
    use crate::state::tag::TagIndex;
    use crate::state::userpage::UserPageIndex;

    #[test]
    fn should_render_with_empty_state_and_say_no_page_created() {
        let result = generate_overview_page(
            &TagIndex {
                entries: HashMap::new(),
            },
            &UserPageIndex {
                entries: HashMap::new(),
            },
        );
        assert_eq!(result.blocks.len(), 1);

        block_contains_markdown_text(result.blocks.get(0).unwrap(), "No tags or pages found!", 0);
    }

    #[test]
    fn should_render_with_tags_and_no_page() {
        let mut entries = HashMap::new();
        entries.insert(
            user_page_id("target"),
            vec![user_page_id("source")].into_iter().collect(),
        );
        let result = generate_overview_page(
            &TagIndex { entries },
            &UserPageIndex {
                entries: HashMap::new(),
            },
        );
        assert_eq!(result.blocks.len(), 1);

        let first_block = result.blocks.get(0).unwrap();
        assert_eq!(first_block.indentation, 0);
        assert_eq!(first_block.content.len(), 1);
        let second_block_content = first_block.content.first().unwrap();
        assert_eq!(second_block_content.as_text, "");
        assert_eq!(second_block_content.as_tokens, vec![
            BlockToken {
                payload: "| pagename | number of backlinks | page has content |\n| :-- | :-- | :-- |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "| ".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "target".to_string(),
                block_token_type: BlockTokenType::Link,
            },
            BlockToken {
                payload: " | 1 | not yet |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
        ]);
    }

    #[test]
    fn should_render_with_tags_and_linked_page() {
        let mut entries = HashMap::new();
        entries.insert(
            user_page_id("target"),
            vec![user_page_id("source")].into_iter().collect(),
        );
        let mut data = HashMap::new();
        data.insert(
            page_name_str("target"),
            ParsedMarkdownFile { blocks: vec![] },
        );
        let result =
            generate_overview_page(&TagIndex { entries }, &UserPageIndex { entries: data });
        assert_eq!(result.blocks.len(), 1);

        let first_block = result.blocks.get(0).unwrap();
        assert_eq!(first_block.indentation, 0);
        assert_eq!(first_block.content.len(), 1);
        let second_block_content = first_block.content.first().unwrap();
        assert_eq!(second_block_content.as_text, "");
        assert_eq!(second_block_content.as_tokens, vec![
            BlockToken {
                payload: "| pagename | number of backlinks | page has content |\n| :-- | :-- | :-- |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "| ".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "target".to_string(),
                block_token_type: BlockTokenType::Link,
            },
            BlockToken {
                payload: " | 1 | yes |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
        ]);
    }
    #[test]
    fn should_append_non_referenced_page() {
        let mut all_pages = HashMap::new();
        all_pages.insert(
            page_name_str("MyPage"),
            ParsedMarkdownFile { blocks: vec![] },
        );
        let result = generate_overview_page(
            &TagIndex {
                entries: HashMap::new(),
            },
            &UserPageIndex { entries: all_pages },
        );
        assert_eq!(result.blocks.len(), 1);

        let first_block = result.blocks.get(0).unwrap();
        assert_eq!(first_block.indentation, 0);
        assert_eq!(first_block.content.len(), 1);
        let second_block_content = first_block.content.first().unwrap();
        assert_eq!(second_block_content.as_text, "");
        assert_eq!(second_block_content.as_tokens, vec![
            BlockToken {
                payload: "| pagename | number of backlinks | page has content |\n| :-- | :-- | :-- |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "| ".to_string(),
                block_token_type: BlockTokenType::Text,
            },
            BlockToken {
                payload: "MyPage".to_string(),
                block_token_type: BlockTokenType::Link,
            },
            BlockToken {
                payload: " | 0 | yes |\n".to_string(),
                block_token_type: BlockTokenType::Text,
            },
        ]);
    }

    fn block_contains_markdown_text(block: &ParsedBlock, markdown_text: &str, indentation: usize) {
        assert_eq!(block.indentation, indentation);
        assert_eq!(block.content.len(), 1);
        let first_content = block.content.first().unwrap();
        assert_eq!(first_content.as_text, "");
        assert_eq!(first_content.as_tokens.len(), 1);
        let first_token = first_content.as_tokens.first().unwrap();
        assert_eq!(first_token.block_token_type, BlockTokenType::Text);
        assert_eq!(first_token.payload, markdown_text);
    }
}
