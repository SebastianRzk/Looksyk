use crate::looksyk::model::{
    BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, SimplePageName,
};
use crate::looksyk::query::render_query;
use crate::looksyk::renderer::atomics::{
    render_journal_link, render_user_link, serialize_reference,
};
use crate::looksyk::renderer::model::{RenderResult, StaticRenderContext};
use crate::looksyk::syntax::looksyk_markdown::render_as_todo_without_padding;
use crate::state::application_state::GraphRootLocation;
use crate::state::asset_cache::AssetCache;

pub fn render_file(
    markdown_file: &ParsedMarkdownFile,
    render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    data_root_location: &GraphRootLocation,
) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block(
            original_block,
            render_context,
            asset_cache,
            data_root_location,
        ));
    }
    PreparedMarkdownFile {
        blocks: result_blocks,
    }
}

pub fn render_block(
    block: &ParsedBlock,
    render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    data_root_location: &GraphRootLocation,
) -> PreparedBlock {
    let mut block_content_original_list = vec![];
    let mut block_content_markdown_list = vec![];
    let mut references = vec![];
    let mut has_dynamic_content = false;

    for content_element in &block.content {
        block_content_original_list.push(content_element.as_text.to_string());
        let render_result = render_tokens_deep(
            &content_element.as_tokens,
            render_context,
            asset_cache,
            data_root_location,
        );
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

pub fn render_tokens_deep(
    tokens: &Vec<BlockToken>,
    render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    data_root_location: &GraphRootLocation,
) -> RenderResult {
    let mut inline_markdown_result_list = vec![];
    let mut references = vec![];
    let mut has_dynamic_content = false;
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
                let render_result =
                    render_query(token, render_context, asset_cache, data_root_location);
                has_dynamic_content = render_result.has_dynamic_content;
                for reference in render_result.referenced_markdown {
                    references.push(reference);
                }
                inline_markdown_result_list.push(render_result.inplace_markdown);
            }
            BlockTokenType::Todo => {
                inline_markdown_result_list.push(render_as_todo_without_padding(token));
            }
        }
    }
    RenderResult {
        has_dynamic_content,
        referenced_markdown: references,
        inline_markdown: inline_markdown_result_list.join(" "),
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::{journal_link_token, link_token, text_token_str};
    use crate::looksyk::index::asset::create_empty_asset_cache;
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock};
    use crate::looksyk::renderer::model::builder::create_empty_render_context;
    use crate::looksyk::renderer::renderer_deep::render_block;
    use crate::state::application_state::builder::empty_data_root_location;

    #[test]
    fn should_serialize_original_text() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![text_token_str("text")],
                as_text: "text".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "text");
        assert_eq!(result.content.prepared_markdown, "text")
    }

    #[test]
    fn should_serialize_links() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    text_token_str("before"),
                    link_token("MyPage"),
                    text_token_str("after"),
                ],
                as_text: "before [[MyPage]] after".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "before [[MyPage]] after");
        assert_eq!(
            result.content.prepared_markdown,
            "before [MyPage](page/MyPage) after"
        )
    }

    #[test]
    fn should_serialize_journal_links() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    text_token_str("before"),
                    journal_link_token("2022_10_30"),
                    text_token_str("after"),
                ],
                as_text: "before [[MyPage]] after".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "before [[MyPage]] after");
        assert_eq!(
            result.content.prepared_markdown,
            "before [30.10.2022](journal/2022_10_30) after"
        )
    }

    #[test]
    fn should_url_encode_link() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![link_token("My Page")],
                as_text: "[[My Page]]".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "[[My Page]]");
        assert_eq!(
            result.content.prepared_markdown,
            "[My Page](page/My%20Page)"
        );
    }

    #[test]
    fn should_render_multiple_links_separately() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    link_token("link1"),
                    text_token_str("asdf"),
                    link_token("link2"),
                ],
                as_text: "[[link1]] asdf [[link2]]".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "[[link1]] asdf [[link2]]");
        assert_eq!(
            result.content.prepared_markdown,
            "[link1](page/link1) asdf [link2](page/link2)"
        );
    }

    #[test]
    fn should_render_text_link_todo_as_static_content() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    BlockToken {
                        payload: does_not_matter(),
                        block_token_type: BlockTokenType::Link,
                    },
                    BlockToken {
                        payload: does_not_matter(),
                        block_token_type: BlockTokenType::Text,
                    },
                    BlockToken {
                        payload: does_not_matter(),
                        block_token_type: BlockTokenType::Todo,
                    },
                ],
                as_text: does_not_matter(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert!(!result.has_dynamic_content);
    }

    #[test]
    fn should_render_unknown_query_as_non_dynamic() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: does_not_matter(),
                    block_token_type: BlockTokenType::Query,
                }],
                as_text: does_not_matter(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert!(!result.has_dynamic_content);
    }

    #[test]
    fn should_render_todo_query_as_dynamic() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "todos tag:\"myTag\" state:\"todo\" display:\"referenced-list\"}"
                        .to_string(),
                    block_token_type: BlockTokenType::Query,
                }],
                as_text: does_not_matter(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert!(result.has_dynamic_content);
    }

    fn does_not_matter() -> String {
        "".to_string()
    }

    #[test]
    fn should_render_checkbox_unchecked() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    BlockToken {
                        payload: " ".to_string(),
                        block_token_type: BlockTokenType::Todo,
                    },
                    text_token_str("Mein Todo"),
                ],
                as_text: "[ ] Mein Todo".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "[ ] Mein Todo");
        assert_eq!(result.content.prepared_markdown, "[ ] Mein Todo");
    }

    #[test]
    fn should_render_checkbox_checked() {
        let input = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![
                    BlockToken {
                        payload: "x".to_string(),
                        block_token_type: BlockTokenType::Todo,
                    },
                    BlockToken {
                        payload: "Mein Todo".to_string(),
                        block_token_type: BlockTokenType::Text,
                    },
                ],
                as_text: "[x] Mein Todo".to_string(),
            }],
        };

        let result = render_block(
            &input,
            &create_empty_render_context().to_static(),
            &mut create_empty_asset_cache(),
            &empty_data_root_location(),
        );

        assert_eq!(result.content.original_text, "[x] Mein Todo");
        assert_eq!(result.content.prepared_markdown, "[x] Mein Todo");
    }
}
