use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, SimplePageName,
};
use crate::looksyk::renderer::atomics::{combine_text_content, render_journal_link};
use crate::looksyk::syntax::looksyk_markdown::{render_as_tag_str, render_as_todo_without_padding};

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
                inline_markdown_result_list.push(render_as_tag_str(&token.payload).to_string());
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
                inline_markdown_result_list
                    .push(render_as_todo_without_padding(token).to_string());
            }
        }
    }
    inline_markdown_result_list.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::builder::{link_token, text_token_str};
    use crate::looksyk::model::{BlockContent, ParsedMarkdownFile};

    #[test]
    fn test_render_file_basic_markdown() {
        let markdown_file = ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_text: "Test content".to_string(),
                    as_tokens: vec![text_token_str("Test content")],
                }],
            }],
        };
        let rendered = render_file_basic_markdown(&markdown_file);
        assert_eq!(rendered.blocks.len(), 1);
        assert_eq!(rendered.blocks[0].content.prepared_markdown, "Test content");
    }

    #[test]
    fn test_render_file_basic_markdown_should_preserve_links_as_tag_link() {
        let markdown_file = ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_text: "Test content".to_string(),
                    as_tokens: vec![link_token("Test link")],
                }],
            }],
        };
        let rendered = render_file_basic_markdown(&markdown_file);
        assert_eq!(rendered.blocks.len(), 1);
        assert_eq!(
            rendered.blocks[0].content.prepared_markdown,
            "[[Test link]]"
        );
    }
}
