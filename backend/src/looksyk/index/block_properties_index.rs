use crate::state::block_properties::BlockPropertiesIndex;
use crate::state::journal::JournalPageIndex;
use crate::state::userpage::UserPageIndex;

pub fn create_block_properties_index(
    journal_pages: &JournalPageIndex,
    user_pages: &UserPageIndex,
) -> BlockPropertiesIndex {
    let mut index = BlockPropertiesIndex {
        entries: std::collections::HashMap::new(),
    };
    for (journal_page_name, journal_page) in journal_pages.entries.iter() {
        for (block_index, block) in journal_page.blocks.iter().enumerate() {
            for property in block.properties.properties.iter() {
                let block_reference = crate::state::block::BlockReference {
                    page_id: journal_page_name.as_journal_page(),
                    block_number: block_index,
                };
                index.append_elements(block_reference, property.clone());
            }
        }
    }

    for (user_page_name, user_page) in user_pages.entries.iter() {
        for (block_index, block) in user_page.blocks.iter().enumerate() {
            for property in block.properties.properties.iter() {
                let block_reference = crate::state::block::BlockReference {
                    page_id: user_page_name.as_user_page(),
                    block_number: block_index,
                };
                index.append_elements(block_reference, property.clone());
            }
        }
    }

    index
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
                block_property_value("key3", journal_page_id("journal-page-name").block_reference(1)),
                block_property_value("value2", user_page_id("user-page-name").block_reference(0)),
            ]
        );
    }
}
