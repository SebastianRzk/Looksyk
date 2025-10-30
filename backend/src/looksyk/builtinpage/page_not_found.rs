use crate::looksyk::model::{ParsedBlock, ParsedMarkdownFile};

pub fn generate_page_not_found() -> ParsedMarkdownFile {
    ParsedMarkdownFile {
        blocks: vec![ParsedBlock::artificial_text_block(
            "Page not (yet) created. Click here to write content!",
        )],
    }
}
