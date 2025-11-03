pub mod models;
pub mod renderer;

use crate::looksyk::kanban::models::{KanbanData, KanbanItem, KanbanItemPriority, KanbanTitle};
use crate::looksyk::model::{ParsedBlock, ParsedMarkdownFile, ReferencedMarkdown, SimplePageName};
use crate::state::block_properties::{
    BlockPropertiesIndex, BlockPropertyKey, BlockPropertyOccurence, BlockPropertyValue,
};
use crate::state::markdown_file::MarkdownFileIndex;

pub fn get_kanban_from_tag(
    title: KanbanTitle,
    tag: SimplePageName,
    column_identifier: BlockPropertyKey,
    column_values: Vec<BlockPropertyValue>,
    block_properties_index: &BlockPropertiesIndex,
    block_property_priority_key: &BlockPropertyKey,
    markdown_file_index: &MarkdownFileIndex,
) -> KanbanData {
    let mut result = KanbanData {
        title,
        lists: vec![],
    };

    for column_value in column_values {
        let mut items = vec![];

        if let Some(occurences) = block_properties_index.entries.get(&column_identifier) {
            for occurence in occurences {
                if occurence.value == column_value
                    && block_contains_tag(&tag, occurence, markdown_file_index)
                {
                    let block = markdown_file_index
                        .resolve(&occurence.block.page_id)
                        .and_then(|page: &ParsedMarkdownFile| {
                            page.block(occurence.block.block_number)
                        })
                        .unwrap();
                    items.push(KanbanItem {
                        block: convert_to_referenced_markdown(occurence, block),
                        priority: extract_priority(block, block_property_priority_key),
                    });
                }
            }
        }

        result
            .lists
            .push(crate::looksyk::kanban::models::KanbanList {
                title: crate::looksyk::kanban::models::KanbanListTitle {
                    title: column_value.value.clone(),
                },
                items,
            });
    }
    result
}

fn convert_to_referenced_markdown(
    occurance: &BlockPropertyOccurence,
    block: &ParsedBlock,
) -> ReferencedMarkdown {
    ReferencedMarkdown {
        content: block.clone(),
        reference: occurance.block.clone(),
    }
}

fn extract_priority(block: &ParsedBlock, priority_key: &BlockPropertyKey) -> KanbanItemPriority {
    KanbanItemPriority {
        priority: block
            .properties
            .properties
            .iter()
            .find(|x| x.key == priority_key.value)
            .map(|x| x.value.clone())
            .unwrap_or("".to_string()),
    }
}

fn block_contains_tag(
    tag: &SimplePageName,
    occurence: &BlockPropertyOccurence,
    markdown_file_index: &MarkdownFileIndex,
) -> bool {
    if occurence.block.page_id.is_user_page() && &occurence.block.page_id.name == tag {
        return true;
    }

    let loaded_block: Option<&ParsedBlock> = markdown_file_index
        .resolve(&occurence.block.page_id)
        .and_then(|page: &ParsedMarkdownFile| page.block(occurence.block.block_number));

    if let Some(loaded_block) = loaded_block {
        if loaded_block.contains_reference(tag) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::{
        any_page_id, empty_journal_index, empty_markdown_file_index,
    };
    use crate::looksyk::builder::{link_token, page_name_str};
    use crate::looksyk::kanban::models::builder::kanban_title;
    use crate::looksyk::kanban::{block_contains_tag, get_kanban_from_tag};
    use crate::looksyk::model::builder::{
        block_with_link_content, block_with_property, block_with_text_content,
    };
    use crate::looksyk::model::{BlockContent, ParsedBlock, ParsedMarkdownFile};
    use crate::looksyk::parser::builder::{block_properties_from, block_property};
    use crate::state::block::BlockReference;
    use crate::state::block_properties::builder::{
        block_property_key, block_property_occurance, block_property_value,
    };
    use crate::state::block_properties::{BlockPropertiesIndex, BlockPropertyKey};
    use crate::state::userpage::builder::{empty_user_page_index, user_page_index};

    #[test]
    fn test_block_contains_tag_with_no_tag() {
        let result = block_contains_tag(
            &page_name_str("non-existent-tag"),
            &block_property_occurance(
                "some-value",
                BlockReference {
                    page_id: any_page_id(),
                    block_number: 0,
                },
            ),
            &empty_markdown_file_index(&empty_journal_index(), &empty_user_page_index()),
        );

        assert!(!result);
    }

    #[test]
    fn test_block_contains_tag_with_matching_tag_as_filename() {
        let page_name = page_name_str("some-value");
        let result = block_contains_tag(
            &page_name,
            &block_property_occurance(
                "some-value",
                BlockReference {
                    page_id: page_name.as_user_page(),
                    block_number: 0,
                },
            ),
            &empty_markdown_file_index(&empty_journal_index(), &empty_user_page_index()),
        );

        assert!(result);
    }

    #[test]
    fn test_block_contains_tag_with_matching_tag() {
        let name = page_name_str("page-name");
        let tag = page_name_str("tag");
        let result = block_contains_tag(
            &tag,
            &block_property_occurance(
                "some-value",
                BlockReference {
                    page_id: name.as_user_page(),
                    block_number: 0,
                },
            ),
            &empty_markdown_file_index(
                &empty_journal_index(),
                &user_page_index(
                    &name.name,
                    ParsedMarkdownFile {
                        blocks: vec![block_with_link_content(&tag.name)],
                    },
                ),
            ),
        );

        assert!(result);
    }

    #[test]
    fn test_extract_priority_with_no_priority() {
        let block = block_with_text_content("");
        let priority_key = BlockPropertyKey {
            value: "any".to_string(),
        };

        let priority = super::extract_priority(&block, &priority_key);

        assert_eq!(priority.priority, "");
    }

    #[test]
    fn test_extract_priority_with_priority() {
        let block = block_with_property("priority", "high");
        let priority_key = BlockPropertyKey {
            value: "priority".to_string(),
        };

        let priority = super::extract_priority(&block, &priority_key);
        assert_eq!(priority.priority, "high");
    }

    #[test]
    fn test_get_kanban_from_tag() {
        let result = get_kanban_from_tag(
            kanban_title("Test Kanban"),
            page_name_str("tag"),
            block_property_key("status"),
            vec![block_property_value("To Do"), block_property_value("Done")],
            &{
                let mut index = BlockPropertiesIndex {
                    entries: std::collections::HashMap::new(),
                };
                index.entries.insert(
                    block_property_key("status"),
                    vec![block_property_occurance(
                        "To Do",
                        page_name_str("page-1").as_user_page().block_reference(0),
                    )],
                );
                index
            },
            &block_property_key("priority"),
            &empty_markdown_file_index(
                &empty_journal_index(),
                &user_page_index(
                    "page-1",
                    ParsedMarkdownFile {
                        blocks: vec![ParsedBlock {
                            content: vec![BlockContent {
                                as_tokens: vec![link_token("tag")],
                                as_text: "".to_string(),
                            }],
                            indentation: 0,
                            properties: block_properties_from(vec![
                                block_property("priority", "high"),
                                block_property("status", "To Do"),
                            ]),
                        }],
                    },
                ),
            ),
        );

        assert_eq!(result.title.title, "Test Kanban");
        assert_eq!(result.lists.len(), 2);
        assert_eq!(result.lists[0].title.title, "To Do");
        assert_eq!(result.lists[0].items.len(), 1);
        assert_eq!(result.lists[0].items[0].priority.priority, "high");
        assert_eq!(result.lists[1].title.title, "Done");
        assert_eq!(result.lists[1].items.len(), 0);
    }
}
