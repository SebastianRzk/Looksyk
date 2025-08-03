use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, SimplePageName,
};
use crate::looksyk::renderer::atomics::{
    combine_text_content, render_journal_link, render_user_link,
};
use crate::looksyk::syntax::looksyk_markdown::render_as_todo_without_padding;

pub fn render_tokens_flat(tokens: &Vec<BlockToken>) -> String {
    let mut inline_markdown_result_list = vec![];
    for token in tokens {
        match token.block_token_type {
            BlockTokenType::Text => {
                inline_markdown_result_list.push(token.payload.clone());
            }
            BlockTokenType::Link => {
                inline_markdown_result_list.push(render_user_link(&SimplePageName {
                    name: token.payload.clone(),
                }));
            }
            BlockTokenType::JournalLink => {
                inline_markdown_result_list.push(render_journal_link(&SimplePageName {
                    name: token.payload.clone(),
                }));
            }
            BlockTokenType::Query => {
                inline_markdown_result_list.push("query hidden".to_string());
            }
            BlockTokenType::Todo => {
                inline_markdown_result_list.push(render_as_todo_without_padding(token).to_string());
            }
        }
    }
    inline_markdown_result_list.join(" ")
}

pub fn render_block_flat(block: &ParsedBlock) -> PreparedBlock {
    PreparedBlock {
        indentation: block.indentation,
        content: render_block_content_flat(block),
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

pub fn render_block_content_flat(block: &ParsedBlock) -> PreparedBlockContent {
    PreparedBlockContent {
        prepared_markdown: render_block_flat_as_string(block),
        original_text: combine_text_content(block),
    }
}

pub fn render_block_flat_as_string(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(render_tokens_flat(&content.as_tokens));
    }
    result_list.join("\n")
}

pub fn render_file_flat(markdown_file: &ParsedMarkdownFile) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block_flat(original_block));
    }
    PreparedMarkdownFile {
        blocks: result_blocks,
    }
}
