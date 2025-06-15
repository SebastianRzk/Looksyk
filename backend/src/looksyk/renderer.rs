extern crate urlencoding;

use crate::io::markdown::markdown_link;
use crate::looksyk::model::{
    BlockToken, BlockTokenType, PageId, PageType, ParsedBlock, ParsedMarkdownFile, PreparedBlock,
    PreparedBlockContent, PreparedMarkdownFile, PreparedReferencedMarkdown, ReferencedMarkdown,
    SimplePageName,
};
use crate::looksyk::query::render_query;
use crate::state::application_state::GraphRootLocation;
use crate::state::asset_cache::AssetCache;
use crate::state::block::BlockReference;
use crate::state::journal::JournalPageIndex;
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;
use crate::state::userpage::UserPageIndex;
use urlencoding::encode;

pub struct StaticRenderContext<'a> {
    pub user_pages: &'a UserPageIndex,
    pub journal_pages: &'a JournalPageIndex,
    pub todo_index: &'a TodoIndex,
    pub tag_index: &'a TagIndex,
}

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

pub fn render_file_flat(markdown_file: &ParsedMarkdownFile) -> PreparedMarkdownFile {
    let mut result_blocks = vec![];
    for original_block in &markdown_file.blocks {
        result_blocks.push(render_block_flat(original_block));
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

fn serialize_reference(referenced_markdown: &ReferencedMarkdown) -> PreparedReferencedMarkdown {
    PreparedReferencedMarkdown {
        reference: BlockReference {
            block_number: referenced_markdown.reference.block_number,
            page_id: referenced_markdown.reference.page_id.clone(),
        },
        content: render_block_content_flat(&referenced_markdown.content),
    }
}

fn combine_text_content(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(content.as_text.clone());
    }
    result_list.join("\n")
}

pub fn render_block_flat(block: &ParsedBlock) -> PreparedBlock {
    PreparedBlock {
        indentation: block.indentation,
        content: render_block_content_flat(block),
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

fn render_block_content_flat(block: &ParsedBlock) -> PreparedBlockContent {
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
                inline_markdown_result_list.push(format!("[{}]", token.payload).to_string());
            }
        }
    }
    inline_markdown_result_list.join(" ")
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
                inline_markdown_result_list.push(format!("[{}]", token.payload));
            }
        }
    }
    RenderResult {
        has_dynamic_content,
        referenced_markdown: references,
        inline_markdown: inline_markdown_result_list.join(" "),
    }
}

pub struct RenderResult {
    inline_markdown: String,
    referenced_markdown: Vec<ReferencedMarkdown>,
    has_dynamic_content: bool,
}

pub fn render_link(destination: &PageId) -> String {
    match destination.page_type {
        PageType::UserPage => render_user_link(&destination.name),
        PageType::JournalPage => render_journal_link(&destination.name),
    }
}

pub fn render_link_by_id(destination: &PageId) -> String {
    match destination.page_type {
        PageType::UserPage => render_user_link(&destination.name),
        PageType::JournalPage => render_journal_link(&destination.name),
    }
}

pub fn render_user_link(destination: &SimplePageName) -> String {
    markdown_link(
        &decode_destination(&destination.name),
        &page_path(&destination),
    )
}

fn decode_destination(destination: &str) -> String {
    destination.replace("%2F", "/")
}

fn decode_date(destination: &str) -> String {
    let splitted = destination.split('_').collect::<Vec<&str>>();
    let mut date = String::new();
    date.push_str(splitted[2]);
    date.push('.');
    date.push_str(splitted[1]);
    date.push('.');
    date.push_str(splitted[0]);
    date
}

fn render_journal_link(destination: &SimplePageName) -> String {
    markdown_link(&decode_date(&destination.name), &journal_path(&destination))
}

pub fn render_block_link(block_reference: &BlockReference) -> String {
    let name = &block_reference.page_id.name;
    let link = match block_reference.page_id.page_type {
        PageType::UserPage => page_path(&name),
        PageType::JournalPage => journal_path(&name),
    };

    markdown_link(
        &format!(
            "{}:{}",
            decode_destination(&name.name),
            block_reference.block_number
        ),
        &link,
    )
}

fn journal_path(name: &&SimplePageName) -> String {
    format!("journal/{}", encode(&name.name))
}

fn page_path(name: &&SimplePageName) -> String {
    format!("page/{}", encode(&name.name))
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::test_builder::empty_journal_index;
    use crate::looksyk::renderer::StaticRenderContext;
    use crate::state::journal::JournalPageIndex;
    use crate::state::tag::builder::empty_tag_index;
    use crate::state::tag::TagIndex;
    use crate::state::todo::builder::empty_todo_index;
    use crate::state::todo::TodoIndex;
    use crate::state::userpage::builder::empty_user_page_index;
    use crate::state::userpage::UserPageIndex;

    pub struct TestRenderContext {
        pub user_pages: UserPageIndex,
        pub journal_pages: JournalPageIndex,
        pub todo_index: TodoIndex,
        pub tag_index: TagIndex,
    }

    impl TestRenderContext {
        pub fn to_static(&self) -> StaticRenderContext {
            StaticRenderContext {
                user_pages: &self.user_pages,
                todo_index: &self.todo_index,
                tag_index: &self.tag_index,
                journal_pages: &self.journal_pages,
            }
        }
    }

    pub fn create_render_context_with_user_page_index(
        user_page_index: UserPageIndex,
    ) -> TestRenderContext {
        TestRenderContext {
            user_pages: user_page_index,
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index: empty_tag_index(),
        }
    }

    pub fn create_render_context_with_todo_index(todo_index: TodoIndex) -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index,
            tag_index: empty_tag_index(),
        }
    }

    pub fn create_render_context_with_tag_index(tag_index: TagIndex) -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index,
        }
    }

    pub fn create_render_context(
        user_page_index: UserPageIndex,
        todo_index: TodoIndex,
        tag_index: TagIndex,
    ) -> TestRenderContext {
        TestRenderContext {
            user_pages: user_page_index,
            journal_pages: empty_journal_index(),
            todo_index,
            tag_index,
        }
    }

    pub fn create_empty_render_context() -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index: empty_tag_index(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::{journal_link_token, link_token, text_token_str};
    use crate::looksyk::index::asset::create_empty_asset_cache;
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock};
    use crate::looksyk::renderer::builder::create_empty_render_context;
    use crate::looksyk::renderer::render_block;
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
