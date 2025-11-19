use crate::io::markdown::markdown_link;
use crate::looksyk::model::{
    PageId, PageType, ParsedBlock, PreparedReferencedMarkdown, ReferencedMarkdown, SimplePageName,
};
use crate::looksyk::renderer::renderer_flat::render_block_content_flat;
use crate::state::block::BlockReference;
use urlencoding::encode;

pub fn render_link(destination: &PageId) -> String {
    match destination.page_type {
        PageType::UserPage => render_user_link(&destination.name),
        PageType::JournalPage => render_journal_link(&destination.name),
    }
}

pub fn render_user_link(destination: &SimplePageName) -> String {
    markdown_link(
        &decode_destination(&destination.name),
        &user_page_path(destination),
    )
}
pub fn serialize_reference(referenced_markdown: &ReferencedMarkdown) -> PreparedReferencedMarkdown {
    PreparedReferencedMarkdown {
        reference: referenced_markdown
            .reference
            .page_id
            .block_reference(referenced_markdown.reference.block_number),
        content: render_block_content_flat(&referenced_markdown.content),
    }
}

pub fn render_journal_link(destination: &SimplePageName) -> String {
    markdown_link(&decode_date(&destination.name), &journal_path(destination))
}

pub fn render_block_link(block_reference: &BlockReference) -> String {
    let name = &block_reference.page_id.name;
    let link = match block_reference.page_id.page_type {
        PageType::UserPage => user_page_path(name),
        PageType::JournalPage => journal_path(name),
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

pub fn journal_path(name: &SimplePageName) -> String {
    format!("journal/{}", encode(&name.name))
}

pub fn user_page_path(name: &SimplePageName) -> String {
    format!("page/{}", encode(&name.name))
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

pub fn combine_text_content(block: &ParsedBlock) -> String {
    let mut result_list = vec![];

    for content in &block.content {
        result_list.push(content.as_text.clone());
    }
    result_list.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::builder::test_builder::{journal_page_id, user_page_id};
    use crate::looksyk::model::PreparedBlockContent;

    #[test]
    fn test_render_link_with_user_page() {
        let user_page = PageId {
            name: SimplePageName {
                name: "Test/Page".to_string(),
            },
            page_type: PageType::UserPage,
        };
        assert_eq!(render_link(&user_page), "[Test/Page](page/Test%2FPage)");
    }

    #[test]
    fn test_render_link_with_journal_page() {
        let journal_page = PageId {
            name: SimplePageName {
                name: "2023_10_01".to_string(),
            },
            page_type: PageType::JournalPage,
        };

        assert_eq!(
            render_link(&journal_page),
            "[01.10.2023](journal/2023_10_01)"
        );
    }

    #[test]
    fn test_serialize_reference() {
        let reference = ReferencedMarkdown {
            reference: BlockReference {
                block_number: 1,
                page_id: user_page_id("Test/Page"),
            },
            content: ParsedBlock::text_block_on_disk("Content"),
        };

        let serialized = serialize_reference(&reference);
        assert_eq!(serialized.reference.block_number, 1);
        assert_eq!(serialized.reference.page_id.name.name, "Test/Page");
        assert_eq!(
            serialized.content,
            PreparedBlockContent {
                prepared_markdown: "Content".to_string(),
                original_text: "Content".to_string(),
            }
        );
    }

    #[test]
    fn test_render_block_link_user_page() {
        let block_reference = BlockReference {
            block_number: 42,
            page_id: user_page_id("Test/Page"),
        };
        let link = render_block_link(&block_reference);
        assert_eq!(link, "[Test/Page:42](page/Test%2FPage)");
    }
    #[test]
    fn test_render_block_link_journal_page() {
        let block_reference = BlockReference {
            block_number: 42,
            page_id: journal_page_id("2024-10-24"),
        };
        let link = render_block_link(&block_reference);
        assert_eq!(link, "[2024-10-24:42](journal/2024-10-24)");
    }

    #[test]
    fn test_decode_destination() {
        assert_eq!(decode_destination("Test%2FPage"), "Test/Page");
        assert_eq!(decode_destination("Another%2FExample"), "Another/Example");
    }

    #[test]
    fn test_decode_date() {
        assert_eq!(decode_date("2023_10_01"), "01.10.2023");
        assert_eq!(decode_date("2022_12_25"), "25.12.2022");
    }
}
