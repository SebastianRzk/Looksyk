use crate::looksyk::model::{PageId, ParsedMarkdownFile, SimplePageName};
use crate::state::block_properties::BlockPropertiesIndex;
use crate::state::journal::JournalPageIndex;
use crate::state::userpage::UserPageIndex;
use std::collections::HashMap;

pub fn create_block_properties_index(
    journal_pages: &JournalPageIndex,
    user_pages: &UserPageIndex,
) -> BlockPropertiesIndex {
    let mut index = BlockPropertiesIndex {
        entries: HashMap::new(),
    };
    for (journal_page_name, journal_page) in journal_pages.entries.iter() {
        insert_journal_page_to_block_properties(&mut index, journal_page_name, journal_page);
    }

    for (user_page_name, user_page) in user_pages.entries.iter() {
        insert_user_page_to_block_properties(&mut index, user_page_name, user_page);
    }
    index
}

pub fn insert_user_page_to_block_properties(
    index: &mut BlockPropertiesIndex,
    user_page_name: &SimplePageName,
    user_page: &ParsedMarkdownFile,
) {
    for (block_index, block) in user_page.blocks.iter().enumerate() {
        for property in block.properties.properties.iter() {
            index.append_elements(
                user_page_name.as_user_page().block_reference(block_index),
                property.clone(),
            );
        }
    }
}

pub fn insert_journal_page_to_block_properties(
    index: &mut BlockPropertiesIndex,
    journal_page_name: &SimplePageName,
    journal_page: &ParsedMarkdownFile,
) {
    for (block_index, block) in journal_page.blocks.iter().enumerate() {
        for property in block.properties.properties.iter() {
            index.append_elements(
                journal_page_name
                    .as_journal_page()
                    .block_reference(block_index),
                property.clone(),
            );
        }
    }
}

pub fn remove_file_from_index(
    index: &BlockPropertiesIndex,
    page_id: &PageId,
) -> BlockPropertiesIndex {
    let mut new_index = HashMap::new();
    index.entries.iter().for_each(|(key, property_values)| {
        let filtered: Vec<_> = property_values
            .iter()
            .filter(|property_value| &property_value.block.page_id != page_id)
            .cloned()
            .collect();
        new_index.insert(key.clone(), filtered);
    });
    BlockPropertiesIndex { entries: new_index }
}

#[cfg(test)]
pub mod tests {
    use crate::looksyk::builder::test_builder::{journal_page_id, user_page_id};
    use crate::looksyk::model::builder::block_with_property;
    use crate::looksyk::model::ParsedMarkdownFile;
    use crate::state::block_properties::builder::{block_property_key, block_property_value};
    use crate::state::journal::builder::journal_page_index;
    use crate::state::userpage::builder::user_page_index;

    #[test]
    fn test_create_block_properties_index() {
        let journals = journal_page_index(
            "journal-page-name",
            ParsedMarkdownFile {
                blocks: vec![
                    block_with_property("key1", "value1"),
                    block_with_property("key2", "key3"),
                ],
            },
        );
        let users = user_page_index(
            "user-page-name",
            ParsedMarkdownFile {
                blocks: vec![block_with_property("key2", "value2")],
            },
        );

        let index = super::create_block_properties_index(&journals, &users);

        assert_eq!(index.entries.len(), 2);

        assert_eq!(
            index.entries.get(&block_property_key("key1")).unwrap(),
            &vec![block_property_value(
                "value1",
                journal_page_id("journal-page-name").block_reference(0)
            )]
        );

        assert_eq!(
            index.entries.get(&block_property_key("key2")).unwrap(),
            &vec![
                block_property_value(
                    "key3",
                    journal_page_id("journal-page-name").block_reference(1)
                ),
                block_property_value("value2", user_page_id("user-page-name").block_reference(0)),
            ]
        );
    }

    #[test]
    fn test_remove_file_from_index() {
        let mut index = super::create_block_properties_index(
            &journal_page_index(
                "journal-page-name",
                ParsedMarkdownFile {
                    blocks: vec![
                        block_with_property("key1", "value1"),
                        block_with_property("key2", "key3"),
                    ],
                },
            ),
            &user_page_index(
                "user-page-name",
                ParsedMarkdownFile {
                    blocks: vec![block_with_property("key2", "value2")],
                },
            ),
        );

        let result =
            super::remove_file_from_index(&mut index, &journal_page_id("journal-page-name"));

        assert_eq!(result.entries.len(), 2);

        assert_eq!(
            result.entries.get(&block_property_key("key1")).unwrap(),
            &vec![]
        );

        assert_eq!(
            result.entries.get(&block_property_key("key2")).unwrap(),
            &vec![block_property_value(
                "value2",
                user_page_id("user-page-name").block_reference(0)
            ),]
        );
    }
}
