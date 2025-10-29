use crate::looksyk::builder::text_token_str;
use crate::looksyk::model::{BlockContent, ParsedBlock, ParsedMarkdownFile};

pub fn generate_page_not_found() -> ParsedMarkdownFile {
    ParsedMarkdownFile {
        blocks: vec![ParsedBlock::artificial_text_block("Page not (yet) created. Click here to write content!")],
    }
}
