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

pub fn render_link_by_id(destination: &PageId) -> String {
    match destination.page_type {
        PageType::UserPage => render_user_link(&destination.name),
        PageType::JournalPage => render_journal_link(&destination.name),
    }
}

pub fn render_user_link(destination: &SimplePageName) -> String {
    markdown_link(
        &decode_destination(&destination.name),
        &page_path(destination),
    )
}
pub fn serialize_reference(referenced_markdown: &ReferencedMarkdown) -> PreparedReferencedMarkdown {
    PreparedReferencedMarkdown {
        reference: BlockReference {
            block_number: referenced_markdown.reference.block_number,
            page_id: referenced_markdown.reference.page_id.clone(),
        },
        content: render_block_content_flat(&referenced_markdown.content),
    }
}

pub fn render_journal_link(destination: &SimplePageName) -> String {
    markdown_link(&decode_date(&destination.name), &journal_path(destination))
}

pub fn render_block_link(block_reference: &BlockReference) -> String {
    let name = &block_reference.page_id.name;
    let link = match block_reference.page_id.page_type {
        PageType::UserPage => page_path(name),
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

fn journal_path(name: &SimplePageName) -> String {
    format!("journal/{}", encode(&name.name))
}

fn page_path(name: &SimplePageName) -> String {
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
