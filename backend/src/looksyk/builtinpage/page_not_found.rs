use crate::looksyk::builder::text_token;
use crate::looksyk::model::{BlockContent, ParsedBlock, ParsedMarkdownFile};

pub fn generate_page_not_found() -> ParsedMarkdownFile {
    ParsedMarkdownFile {
        blocks: vec![
            ParsedBlock {
                content: vec![
                    BlockContent {
                        as_text: "".to_string(),
                        as_tokens: vec![text_token("Page not (yet) created. Click here to write content!")],
                    }
                ],
                indentation: 0,
            }
        ]
    }
}