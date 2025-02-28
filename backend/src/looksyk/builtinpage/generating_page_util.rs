use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock};

pub fn create_textblock(text: &str, indentation: usize) -> ParsedBlock {
    ParsedBlock {
        indentation,
        content: vec![BlockContent {
            as_tokens: vec![BlockToken {
                payload: text.to_string(),
                block_token_type: BlockTokenType::Text,
            }],
            as_text: "".to_string(),
        }],
    }
}
