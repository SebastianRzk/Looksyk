use crate::io::fs::pages::PageOnDisk;
use crate::looksyk::model::{PreparedMarkdownFile, RawMarkdownFile};
use crate::looksyk::parser::parse_markdown_file;
use crate::looksyk::reader::parse_lines;
use crate::looksyk::renderer::renderer_basic_markdown::render_file_basic_markdown;

pub fn help_page() -> PreparedMarkdownFile {
    let parsed_lines = parse_lines(help_page_on_disk().content.lines());
    let parsed_file = parse_markdown_file(RawMarkdownFile {
        blocks: parsed_lines,
    });
    render_file_basic_markdown(&parsed_file)
}


fn help_page_on_disk() -> PageOnDisk {
    let help_page_text = include_str!("help_page.md");
    PageOnDisk{
        content: help_page_text.to_string(),
        name: "Help".to_string(),
    }
}