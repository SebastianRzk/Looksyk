use crate::io::human::{filesize_as_human_string, timestamp_as_human_string};
use crate::looksyk::builder::text_token;
use crate::looksyk::model::{no_text_content, BlockContent, ParsedBlock, ParsedMarkdownFile};
use crate::looksyk::parser::BlockProperties;

pub fn get_asset_meta_info_table(size: u64, last_changed: i64) -> ParsedMarkdownFile {
    ParsedMarkdownFile {
        blocks: vec![ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: no_text_content(),
                as_tokens: vec![text_token(format!(
                    "### Properties \n\n\
                             | Property | Value |\n | :-- | :-- |\n\
                             | Size | {} |\n\
                             | Last Changed | {} |",
                    filesize_as_human_string(size),
                    timestamp_as_human_string(last_changed),
                ))],
            }],
            properties: BlockProperties::empty()
        }],
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builtinpage::asset_metainfo_table::get_asset_meta_info_table;
    use crate::looksyk::model::no_text_content;

    #[test]
    fn test_get_asset_meta_info_table() {
        let result = get_asset_meta_info_table(1024, 1610000000);
        assert_eq!(result.blocks.len(), 1);
        assert_eq!(result.blocks[0].content.len(), 1);
        assert_eq!(result.blocks[0].content[0].as_text, no_text_content());

        assert_eq!(result.blocks[0].content[0].as_tokens.len(), 1);
        assert_eq!(
            result.blocks[0].content[0].as_tokens[0].payload,
            "### Properties \n\n\
                             | Property | Value |\n | :-- | :-- |\n\
                             | Size | 1.02 kB |\n\
                             | Last Changed | 07.01.2021 06:13:20 |"
        );
    }
}
