use crate::looksyk::model::{ParsedBlock, ParsedMarkdownFile};

pub fn append_template_to_page(
    block_index: usize,
    template: &ParsedMarkdownFile,
    target: &ParsedMarkdownFile,
) -> ParsedMarkdownFile {
    let block_indentation = if block_index < target.blocks.len() {
        target.blocks[block_index].indentation
    } else {
        0
    };

    let rendered_template = render_template(template, block_indentation);

    append_to_page(target, block_index, rendered_template)
}

fn append_to_page(
    target: &ParsedMarkdownFile,
    block_index: usize,
    template: ParsedMarkdownFile,
) -> ParsedMarkdownFile {
    let mut result = vec![];
    let mut iteration_index = 0;

    if block_index >= target.blocks.len() {
        // If the block index is greater than the number of blocks, append at the end
        return ParsedMarkdownFile {
            blocks: target
                .blocks
                .iter()
                .cloned()
                .chain(template.blocks)
                .collect(),
        };
    }

    for block in &target.blocks {
        let mut result_block = block.clone();
        if iteration_index == block_index {
            let mut first_block = true;
            for template_block in &template.blocks {
                if first_block {
                    let mut first_line = true;
                    for line in &template_block.content {
                        if first_line {
                            if result_block.content.is_empty() {
                                result_block.content.push(line.clone());
                            } else {
                                let len = &result_block.content.len() - 1;
                                result_block.content[len]
                                    .as_tokens
                                    .extend(line.as_tokens.clone());
                                result_block.content[len].as_text.push_str(&line.as_text);
                            }
                            first_line = false;
                            continue;
                        }
                        result_block.content.push(line.clone());
                    }

                    first_block = false;
                    result.push(result_block.clone());
                    continue;
                }
                result.push(template_block.clone());
            }
            continue;
        }
        result.push(result_block);
        iteration_index += 1;
    }

    ParsedMarkdownFile { blocks: result }
}

fn render_template(
    template: &ParsedMarkdownFile,
    target_root_indentation: usize,
) -> ParsedMarkdownFile {
    let mut rendered_blocks = vec![];

    for block in &template.blocks {
        let rendered_block = render_template_block(block, target_root_indentation);
        rendered_blocks.push(rendered_block);
    }

    ParsedMarkdownFile {
        blocks: rendered_blocks,
    }
}

fn render_template_block(
    template_block: &ParsedBlock,
    target_root_indentation: usize,
) -> ParsedBlock {
    ParsedBlock {
        content: template_block.content.clone(),
        indentation: target_root_indentation + template_block.indentation,
        properties: template_block.properties.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::builder::test_builder::block_content;
    use crate::looksyk::builder::text_token_str;
    use crate::looksyk::model::builder::block_with_text_content;
    use crate::looksyk::parser::BlockProperties;

    #[test]
    fn should_append_template_to_page_should_append_last_line() {
        let target = ParsedMarkdownFile {
            blocks: vec![block_with_text_content("existing content")],
        };

        let template = ParsedMarkdownFile {
            blocks: vec![block_with_text_content("template content")],
        };

        let updated_page = append_template_to_page(0, &template, &target);

        assert_eq!(updated_page.blocks.len(), 1);
        assert_eq!(updated_page.blocks[0].content.len(), 1);
        assert_eq!(
            updated_page.blocks[0].content[0].as_tokens,
            vec![
                text_token_str("existing content"),
                text_token_str("template content")
            ]
        );
        assert_eq!(
            updated_page.blocks[0].content[0].as_text,
            "existing contenttemplate content"
        );
    }

    #[test]
    fn should_append_template_to_page_should_append_last_line_and_following_lines_in_own_line() {
        let target = ParsedMarkdownFile {
            blocks: vec![block_with_text_content("existing content")],
        };

        let template = ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![
                    block_content("template content"),
                    block_content("more template content"),
                ],
                properties: BlockProperties::empty(),
            }],
        };

        let updated_page = append_template_to_page(0, &template, &target);

        assert_eq!(updated_page.blocks.len(), 1);
        assert_eq!(updated_page.blocks[0].content.len(), 2);
        assert_eq!(
            updated_page.blocks[0].content[0].as_tokens,
            vec![
                text_token_str("existing content"),
                text_token_str("template content")
            ]
        );
        assert_eq!(
            updated_page.blocks[0].content[0].as_text,
            "existing contenttemplate content"
        );
        assert_eq!(
            updated_page.blocks[0].content[1].as_tokens,
            vec![text_token_str("more template content")]
        );
        assert_eq!(
            updated_page.blocks[0].content[1].as_text,
            "more template content"
        );
    }

    #[test]
    fn should_adjust_indentation_of_folliwing_blocks() {
        let target = ParsedMarkdownFile {
            blocks: vec![ParsedBlock {
                indentation: 1,
                content: vec![block_content("existing content")],

                properties: BlockProperties::empty(),
            }],
        };

        let template = ParsedMarkdownFile {
            blocks: vec![
                ParsedBlock::text_block_on_disk("template content"),
                ParsedBlock {
                    indentation: 2,
                    content: vec![block_content("template content block 2")],
                    properties: BlockProperties::empty(),
                },
            ],
        };
        let updated_page = append_template_to_page(0, &template, &target);

        assert_eq!(updated_page.blocks.len(), 2);
        assert_eq!(updated_page.blocks[0].content.len(), 1);
        assert_eq!(
            updated_page.blocks[0].content[0].as_tokens,
            vec![
                text_token_str("existing content"),
                text_token_str("template content")
            ]
        );
        assert_eq!(
            updated_page.blocks[0].content[0].as_text,
            "existing contenttemplate content"
        );
        assert_eq!(updated_page.blocks[0].indentation, 1);

        assert_eq!(updated_page.blocks[1].content.len(), 1);
        assert_eq!(
            updated_page.blocks[1].content[0].as_tokens,
            vec![text_token_str("template content block 2")]
        );
        assert_eq!(
            updated_page.blocks[1].content[0].as_text,
            "template content block 2"
        );
        assert_eq!(updated_page.blocks[1].indentation, 3);
    }
}
