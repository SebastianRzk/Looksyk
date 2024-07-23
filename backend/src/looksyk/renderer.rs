extern crate urlencoding;

use urlencoding::encode;

use crate::looksyk::model::{BlockToken, BlockTokenType, MarkdownReference, PageId, PageType, ParsedBlock, ParsedMarkdownFile, PreparedBlock, PreparedBlockContent, PreparedMarkdownFile, PreparedReferencedMarkdown, ReferencedMarkdown, SimplePageName};
use crate::looksyk::page_index::{get_page_type, strip_journal_page_prefix, strip_user_page_prefix};
use crate::looksyk::query::render_query;
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;
use crate::state::userpage::UserPageIndex;

pub fn render_file(markdown_file: &ParsedMarkdownFile, data: &UserPageIndex, todo_index: &TodoIndex, tag_index: &TagIndex) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block(original_block, data, todo_index, tag_index));
    }
    PreparedMarkdownFile {
        blocks: result_blocks
    }
}

pub fn render_block(block: &ParsedBlock, data: &UserPageIndex, todo_index: &TodoIndex, tag_index: &TagIndex) -> PreparedBlock {
    let mut block_content_original_list = vec![];
    let mut block_content_markdown_list = vec![];
    let mut references = vec![];
    let mut has_dynamic_content = false;

    for content_element in &block.content {
        block_content_original_list.push(content_element.as_text.to_string());
        let render_result = render_tokens_deep(&content_element.as_tokens, data, todo_index, tag_index);
        if render_result.has_dynamic_content {
            has_dynamic_content = true;
        }
        block_content_markdown_list.push(render_result.inline_markdown);
        for reference in &render_result.referenced_markdown {
            references.push(serialize_reference(reference));
        }
    }

    PreparedBlock {
        indentation: block.indentation,
        content: PreparedBlockContent {
            prepared_markdown: block_content_markdown_list.join("\n"),
            original_text: block_content_original_list.join("\n"),
        },
        referenced_markdown: references,
        has_dynamic_content,
    }
}

fn serialize_reference(referenced_markdown: &ReferencedMarkdown) -> PreparedReferencedMarkdown {
    PreparedReferencedMarkdown {
        reference: MarkdownReference {
            block_number: referenced_markdown.refernce.block_number,
            page_name: referenced_markdown.refernce.page_name.clone(),
            page_id: referenced_markdown.refernce.page_id.clone(),
        },
        content: PreparedBlockContent {
            original_text: combine_text_content(&referenced_markdown.content),
            prepared_markdown: render_block_flat(&referenced_markdown.content).trim().to_string(),
        },
    }
}


fn combine_text_content(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(content.as_text.clone());
    }
    result_list.join("\n")
}


pub fn render_block_flat(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(render_tokens_flat(&content.as_tokens));
    }
    result_list.join("\n")
}


pub fn render_tokens_flat(tokens: &Vec<BlockToken>) -> String {
    let mut inline_markdown_result_list = vec![];
    for token in tokens {
        match token.block_token_type {
            BlockTokenType::TEXT => {
                inline_markdown_result_list.push(token.payload.clone());
            }
            BlockTokenType::LINK => {
                inline_markdown_result_list.push(render_user_link_str(&token.payload));
            }
            BlockTokenType::JOURNAL_LINK => {
                inline_markdown_result_list.push(render_journal_link_str(&token.payload));
            }
            BlockTokenType::QUERY => {
                inline_markdown_result_list.push("query hidden".to_string());
            }
            BlockTokenType::TODO => {
                inline_markdown_result_list.push(format!("[{}]", token.payload).to_string());
            }
        }
    }
    return inline_markdown_result_list.join(" ");
}

pub fn render_tokens_deep(tokens: &Vec<BlockToken>, data: &UserPageIndex, todo_index: &TodoIndex, tag_index: &TagIndex) -> RenderResult {
    let mut inline_markdown_result_list = vec![];
    let mut references = vec![];
    let mut has_dynamic_content = false;
    for token in tokens {
        match token.block_token_type {
            BlockTokenType::TEXT => {
                inline_markdown_result_list.push(token.payload.clone());
            }
            BlockTokenType::LINK => {
                inline_markdown_result_list.push(render_user_link_str(&token.payload));
            }
            BlockTokenType::JOURNAL_LINK => {
                inline_markdown_result_list.push(render_journal_link_str(&token.payload));
            }
            BlockTokenType::QUERY => {
                has_dynamic_content = true;
                let render_result = render_query(token, data, todo_index, tag_index);
                for reference in render_result.referenced_markdown {
                    references.push(reference);
                }
                inline_markdown_result_list.push(render_result.inplace_markdown);
            }
            BlockTokenType::TODO => {
                inline_markdown_result_list.push(format!("[{}]", token.payload));
            }
        }
    }
    return RenderResult {
        has_dynamic_content,
        referenced_markdown: references,
        inline_markdown: inline_markdown_result_list.join(" "),
    };
}

pub struct RenderResult {
    inline_markdown: String,
    referenced_markdown: Vec<ReferencedMarkdown>,
    has_dynamic_content: bool,
}

pub fn render_link(destination: &SimplePageName, page_type: &PageType) -> String {
    match page_type {
        PageType::UserPage => render_user_link_str(&destination.name),
        PageType::JournalPage => render_journal_link_str(&destination.name)
    }
}


pub fn render_user_link(destination: &SimplePageName) -> String {
    render_user_link_str(&destination.name)
}

pub fn render_link_by_id(destination: &PageId) -> String {
    let page_type = get_page_type(destination);
    match page_type {
        PageType::UserPage => {
            render_user_link_str(&strip_user_page_prefix(destination).name)
        }
        PageType::JournalPage => {
            render_journal_link_str(&strip_journal_page_prefix(destination).name)
        }
    }
}

fn render_user_link_str(destination: &String) -> String {
    format!("[{}](page/{})", decode_destination(destination), encode(destination))
}

fn decode_destination(destination: &String) -> String {
    destination.replace("%2F", "/")
}

fn render_journal_link_str(destination: &String) -> String {
    format!("[{}](journal/{})", decode_destination(destination), encode(destination))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::looksyk::builder::{journal_link_token, link_token, text_token};
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock};
    use crate::looksyk::renderer::render_block;
    use crate::state::tag::TagIndex;
    use crate::state::todo::TodoIndex;
    use crate::state::userpage::UserPageIndex;

    fn empty_todo_index() -> TodoIndex {
        TodoIndex {
            entries: vec![]
        }
    }

    fn empty_tag_index() -> TagIndex {
        TagIndex {
            entries: HashMap::new()
        }
    }

    #[test]
    fn should_serialize_original_text() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        BlockToken {
                            payload: "text".to_string(),
                            block_token_type: BlockTokenType::TEXT,
                        },
                    ],
                    as_text: "text".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "text");
        assert_eq!(result.content.prepared_markdown, "text")
    }


    #[test]
    fn should_serialize_links() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        text_token("before"),
                        link_token("MyPage"),
                        text_token("after"),
                    ],
                    as_text: "before [[MyPage]] after".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "before [[MyPage]] after");
        assert_eq!(result.content.prepared_markdown, "before [MyPage](page/MyPage) after")
    }

    #[test]
    fn should_serialize_journal_links() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        text_token("before"),
                        journal_link_token("MyPage"),
                        text_token("after"),
                    ],
                    as_text: "before [[MyPage]] after".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "before [[MyPage]] after");
        assert_eq!(result.content.prepared_markdown, "before [MyPage](journal/MyPage) after")
    }


    #[test]
    fn should_url_encode_link() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        link_token("My Page"),
                    ],
                    as_text: "[[My Page]]".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "[[My Page]]");
        assert_eq!(result.content.prepared_markdown, "[My Page](page/My%20Page)");
    }

    #[test]
    fn should_render_multiple_links_separately() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        link_token("link1"),
                        text_token("asdf"),
                        link_token("link2"),
                    ],
                    as_text: "[[link1]] asdf [[link2]]".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "[[link1]] asdf [[link2]]");
        assert_eq!(result.content.prepared_markdown, "[link1](page/link1) asdf [link2](page/link2)");
    }


    #[test]
    fn should_render_text_link_todo_as_static_content() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        BlockToken {
                            payload: does_not_matter(),
                            block_token_type: BlockTokenType::LINK,
                        },
                        BlockToken {
                            payload: does_not_matter(),
                            block_token_type: BlockTokenType::TEXT,
                        },
                        BlockToken {
                            payload: does_not_matter(),
                            block_token_type: BlockTokenType::TODO,
                        },
                    ],
                    as_text: does_not_matter(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.has_dynamic_content, false);
    }


    #[test]
    fn should_render_query_as_dynamic() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        BlockToken {
                            payload: does_not_matter(),
                            block_token_type: BlockTokenType::QUERY,
                        },
                    ],
                    as_text: does_not_matter(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.has_dynamic_content, true);
    }


    fn does_not_matter() -> String {
        "".to_string()
    }


    #[test]
    fn should_render_checkbox_unchecked() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        BlockToken {
                            payload: " ".to_string(),
                            block_token_type: BlockTokenType::TODO,
                        },
                        text_token("Mein Todo"),
                    ],
                    as_text: "[ ] Mein Todo".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "[ ] Mein Todo");
        assert_eq!(result.content.prepared_markdown, "[ ] Mein Todo");
    }

    fn empty_page_index() -> UserPageIndex {
        UserPageIndex {
            entries: HashMap::new()
        }
    }

    #[test]
    fn should_render_checkbox_checked() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![
                BlockContent {
                    as_tokens: vec![
                        BlockToken {
                            payload: "x".to_string(),
                            block_token_type: BlockTokenType::TODO,
                        },
                        BlockToken {
                            payload: "Mein Todo".to_string(),
                            block_token_type: BlockTokenType::TEXT,
                        },
                    ],
                    as_text: "[x] Mein Todo".to_string(),
                }
            ],
        };

        let result = render_block(&input, &empty_page_index(), &empty_todo_index(), &empty_tag_index());

        assert_eq!(result.content.original_text, "[x] Mein Todo");
        assert_eq!(result.content.prepared_markdown, "[x] Mein Todo");
    }
}