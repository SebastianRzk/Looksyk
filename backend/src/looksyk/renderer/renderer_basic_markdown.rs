use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, SimplePageName,
};
use crate::looksyk::renderer::atomics::{combine_text_content, render_journal_link};

pub fn render_file_basic_markdown(markdown_file: &ParsedMarkdownFile) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block_basic_markdown(original_block));
    }
    PreparedMarkdownFile {
        blocks: result_blocks,
    }
}

fn render_block_basic_markdown(block: &ParsedBlock) -> PreparedBlock {
    PreparedBlock {
        indentation: block.indentation,
        content: PreparedBlockContent {
            prepared_markdown: render_block_basic_markdown_as_string(block),
            original_text: combine_text_content(block),
        },
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

pub fn render_block_basic_markdown_as_string(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(render_tokens_text_only(&content.as_tokens));
    }
    result_list.join("\n")
}

fn render_tokens_text_only(tokens: &Vec<BlockToken>) -> String {
    let mut inline_markdown_result_list = vec![];
    for token in tokens {
        match token.block_token_type {
            BlockTokenType::Text => {
                inline_markdown_result_list.push(token.payload.clone());
            }
            BlockTokenType::Link => {
                inline_markdown_result_list.push(format!("[[{}]]", token.payload).to_string());
            }
            BlockTokenType::JournalLink => {
                inline_markdown_result_list.push(render_journal_link(&SimplePageName {
                    name: token.payload.clone(),
                }));
            }
            BlockTokenType::Query => {
                inline_markdown_result_list
                    .push(format!("{{query: {}}}", token.payload).to_string());
            }
            BlockTokenType::Todo => {
                inline_markdown_result_list.push(format!("[{}]", token.payload).to_string());
            }
        }
    }
    inline_markdown_result_list.join(" ")
}
