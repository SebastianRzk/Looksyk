use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, SimplePageName,
};
use crate::looksyk::renderer::atomics::{
    combine_text_content, render_journal_link, render_user_link,
};
use crate::looksyk::renderer::title::JournalTitleCalculatorMetadata;
use crate::looksyk::syntax::looksyk_markdown::{render_as_todo_without_padding, render_property};

pub fn render_tokens_flat(
    tokens: &Vec<BlockToken>,
    journal_title_calculator_metadata: &JournalTitleCalculatorMetadata,
) -> String {
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
                inline_markdown_result_list.push(render_journal_link(
                    &SimplePageName {
                        name: token.payload.clone(),
                    },
                    journal_title_calculator_metadata,
                ));
            }
            BlockTokenType::Query => {
                inline_markdown_result_list.push("query hidden".to_string());
            }
            BlockTokenType::Todo => {
                inline_markdown_result_list.push(render_as_todo_without_padding(token).to_string());
            }
            BlockTokenType::Property => {
                inline_markdown_result_list.push(render_property(token));
            }
        }
    }
    inline_markdown_result_list.join(" ")
}

pub fn render_block_flat(
    block: &ParsedBlock,
    journal_title_calculator_metadata: &JournalTitleCalculatorMetadata,
) -> PreparedBlock {
    PreparedBlock {
        indentation: block.indentation,
        content: render_block_content_flat(block, journal_title_calculator_metadata),
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

pub fn render_block_content_flat(
    block: &ParsedBlock,
    journal_title_calculator_metadata: &JournalTitleCalculatorMetadata,
) -> PreparedBlockContent {
    PreparedBlockContent {
        prepared_markdown: render_block_flat_as_string(block, journal_title_calculator_metadata),
        original_text: combine_text_content(block),
    }
}

pub fn render_block_flat_as_string(
    block: &ParsedBlock,
    journal_title_calculator_metadata: &JournalTitleCalculatorMetadata,
) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(render_tokens_flat(
            &content.as_tokens,
            journal_title_calculator_metadata,
        ));
    }
    result_list.join("\n")
}

pub fn render_file_flat(
    markdown_file: &ParsedMarkdownFile,
    journal_title_calculator_metadata: &JournalTitleCalculatorMetadata,
) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block_flat(
            original_block,
            journal_title_calculator_metadata,
        ));
    }
    PreparedMarkdownFile {
        blocks: result_blocks,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::looksyk::model::builder::block_with_block_property_token;
    use crate::looksyk::renderer::renderer_flat::render_tokens_flat;
    use crate::looksyk::renderer::title::builder::world_journal_title_calculator_metadata;

    #[test]
    fn render_property_as_property() {
        let input = block_with_block_property_token("key:: value");

        let result = render_tokens_flat(
            &input.content[0].as_tokens,
            &world_journal_title_calculator_metadata(),
        );

        assert_eq!(result, "<code class=\"inline-property\">key:: value</code>");
    }
}
