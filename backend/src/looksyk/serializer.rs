use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, UpdateBlock,
};
use crate::looksyk::syntax::looksyk_markdown::{
    render_as_query, render_as_tag_str, render_as_todo,
};

pub fn update_and_serialize_page(
    update_block: &UpdateBlock,
    parsed_markdown_file: &ParsedMarkdownFile,
) -> Vec<String> {
    let mut result = vec![];
    for (block_number, block) in parsed_markdown_file.blocks.iter().enumerate() {
        if block_number == update_block.reference.block_number {
            let indentation = block.indentation;

            if update_block.markdown.contains("\n") {
                let mut first = true;
                for line in update_block.markdown.split("\n") {
                    result.push(generate_file_line(indentation, first, line));
                    first = false;
                }
            } else {
                result.push(format!(
                    "{}- {}",
                    generate_indentation(indentation),
                    sanitize_prefix(update_block.markdown.as_str())
                ));
            }
        } else {
            serialize_block_content(&mut result, &block);
        }
    }
    result
}
pub fn serialize_page(parsed_markdown_file: &ParsedMarkdownFile) -> Vec<String> {
    let mut result = vec![];
    for block in &parsed_markdown_file.blocks {
        serialize_block_content(&mut result, &block);
    }
    result
}

fn serialize_block_content(result: &mut Vec<String>, block: &&ParsedBlock) {
    let mut serialized_block_content = vec![];
    for block_content in &block.content {
        let mut block_content_token = vec![];
        for block_token in &block_content.as_tokens {
            block_content_token.push(serialize_block_token(block_token));
        }
        serialized_block_content.push(block_content_token.join(""));
    }
    let serialized_block = serialized_block_content.join("\n");
    let indentation = block.indentation;

    if serialized_block.contains("\n") {
        let mut first = true;
        for line in serialized_block.split("\n") {
            result.push(generate_file_line(indentation, first, line));
            first = false;
        }
    } else {
        result.push(format!(
            "{}- {}",
            generate_indentation(indentation),
            sanitize_prefix(serialized_block.as_str())
        ));
    }
}

fn generate_file_line(indentation: usize, first: bool, line: &str) -> String {
    if first {
        return format!("{}- {}", generate_indentation(indentation), line);
    }
    format!(
        "{}{}",
        generate_indentation(indentation),
        sanitize_prefix(line)
    )
}

fn sanitize_prefix(line: &str) -> String {
    line.trim_start_matches("-").to_string()
}

fn generate_indentation(depth: usize) -> String {
    let mut result = "".to_string();
    for _ in 0..depth {
        result.push('\t');
    }
    result
}

fn serialize_block_token(block_token: &BlockToken) -> String {
    match block_token.block_token_type {
        BlockTokenType::Text => block_token.payload.clone(),
        BlockTokenType::Property => {
            block_token.payload.clone()
        }
        BlockTokenType::Link => render_as_tag_str(&block_token.payload),
        BlockTokenType::JournalLink => {
            format!("[[journal::{}]]", block_token.payload.clone())
        }
        BlockTokenType::Query => render_as_query(block_token),
        BlockTokenType::Todo => render_as_todo(block_token),
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::any_page_id;
    use crate::looksyk::builder::{journal_link_token, text_token_str};
    use crate::looksyk::model::{
        BlockContent, BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, UpdateBlock,
    };
    use crate::looksyk::parser::BlockProperties;
    use crate::looksyk::serializer::{serialize_block_token, update_and_serialize_page};
    use crate::state::block::BlockReference;

    #[test]
    fn should_serialize_text() {
        let block_token = text_token_str("Text1");

        let result = serialize_block_token(&block_token);

        assert_eq!(result, "Text1");
    }

    #[test]
    fn should_serialize_todo() {
        let block_token = todo_block("x");

        let result = serialize_block_token(&block_token);

        assert_eq!(result, "[x] ");
    }

    #[test]
    fn should_serialize_link() {
        let block_token = link_block("my link");

        let result = serialize_block_token(&block_token);

        assert_eq!(result, "[[my link]]");
    }
    #[test]
    fn should_serialize_journal_link() {
        let block_token = journal_link_token("my link");

        let result = serialize_block_token(&block_token);

        assert_eq!(result, "[[journal::my link]]");
    }

    #[test]
    fn should_serialize_query() {
        let block_token = query_block("myquery");

        let result = serialize_block_token(&block_token);

        assert_eq!(result, "{query: myquery }");
    }

    #[test]
    fn should_join_and_update_blocks() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "this is a new content".to_string(),
                reference: BlockReference {
                    block_number: 1,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 0),
                    parsed_text_block("This is line 2", 0),
                ],
            },
        );

        assert_eq!(result, vec!["- This is Line 1", "- this is a new content"])
    }

    #[test]
    fn should_allow_multiline_content_and_should_indent() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "this is a new content\nwith line 2".to_string(),
                reference: BlockReference {
                    block_number: 0,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 1),
                    parsed_text_block("This is line 2\nthis is line 3", 2),
                ],
            },
        );

        assert_eq!(
            result,
            vec![
                "\t- this is a new content",
                "\twith line 2",
                "\t\t- This is line 2",
                "\t\tthis is line 3"
            ]
        )
    }

    #[test]
    fn should_not_join_token_with_space() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "this is a new content".to_string(),
                reference: BlockReference {
                    block_number: 0,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 1),
                    ParsedBlock::from_tokens(vec![
                        todo_block("x"),
                        text_token_str("mytodo "),
                        link_block("my link"),
                    ]),
                ],
            },
        );

        assert_eq!(
            result,
            vec!["\t- this is a new content", "- [x] mytodo [[my link]]"]
        )
    }

    #[test]
    fn should_join_content_with_linebreak() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "this is a new content".to_string(),
                reference: BlockReference {
                    block_number: 0,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 1),
                    ParsedBlock {
                        indentation: 1,
                        content: vec![
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("my text")],
                            },
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("2my text2")],
                            },
                        ],
                        properties: BlockProperties::empty(),
                    },
                ],
            },
        );

        assert_eq!(
            result,
            vec!["\t- this is a new content", "\t- my text", "\t2my text2"]
        )
    }

    #[test]
    fn should_remove_leading_minus_in_block_content_to_prevent_block_injection() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "this is a new\n-content".to_string(),
                reference: BlockReference {
                    block_number: 0,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 1),
                    ParsedBlock {
                        indentation: 1,
                        content: vec![
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("my text")],
                            },
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("-2my text2")],
                            },
                        ],
                        properties: BlockProperties::empty(),
                    },
                ],
            },
        );

        assert_eq!(
            result,
            vec![
                "\t- this is a new",
                "\tcontent",
                "\t- my text",
                "\t2my text2"
            ]
        )
    }

    #[test]
    fn should_handle_trailing_new_lines() {
        let result = update_and_serialize_page(
            &UpdateBlock {
                markdown: "update\n".to_string(),
                reference: BlockReference {
                    block_number: 0,
                    page_id: any_page_id(),
                },
            },
            &ParsedMarkdownFile {
                blocks: vec![
                    parsed_text_block("This is Line 1", 1),
                    ParsedBlock {
                        indentation: 0,
                        content: vec![BlockContent {
                            as_text: "".to_string(),
                            as_tokens: vec![text_token_str("")],
                        }],
                        properties: BlockProperties::empty(),
                    },
                    ParsedBlock {
                        indentation: 1,
                        content: vec![
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("my text")],
                            },
                            BlockContent {
                                as_text: "".to_string(),
                                as_tokens: vec![text_token_str("-2my text2\n\n")],
                            },
                        ],
                        properties: BlockProperties::empty(),
                    },
                ],
            },
        );

        assert_eq!(
            result,
            vec![
                "\t- update",
                "\t",
                "- ",
                "\t- my text",
                "\t2my text2",
                "\t",
                "\t"
            ]
        )
    }

    fn parsed_text_block(text: &str, indentation: usize) -> ParsedBlock {
        ParsedBlock {
            content: vec![BlockContent {
                as_text: "".to_string(),
                as_tokens: vec![text_token_str(text)],
            }],
            indentation,
            properties: BlockProperties::empty(),
        }
    }

    fn todo_block(text: &str) -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Todo,
            payload: text.to_string(),
        }
    }

    fn link_block(text: &str) -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Link,
            payload: text.to_string(),
        }
    }

    fn query_block(text: &str) -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Query,
            payload: text.to_string(),
        }
    }
}
