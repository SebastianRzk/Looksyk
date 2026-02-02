use std::collections::HashMap;

use crate::io::fs::pages::PageOnDisk;
use crate::looksyk::builder::page_name;
use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
use crate::looksyk::parser::parse_markdown_file;
use crate::looksyk::reader::read_file_contents;
use crate::state::journal::JournalPageIndex;
use crate::state::userpage::UserPageIndex;

pub fn create_user_page_index(all_files: &[PageOnDisk]) -> UserPageIndex {
    UserPageIndex {
        entries: parse_files(all_files),
    }
}

pub fn create_journal_page_index(all_files: &[PageOnDisk]) -> JournalPageIndex {
    let mut index: JournalPageIndex = Default::default();

    for file in all_files {
        let raw_markdown_file = read_file_contents(&file.content);
        let parsed_markdown_file = parse_markdown_file(raw_markdown_file);
        index.insert(page_name(file.name.clone()), parsed_markdown_file);
    }

    index
}

fn parse_files(all_files: &[PageOnDisk]) -> HashMap<SimplePageName, ParsedMarkdownFile> {
    let mut parsed_file_index = HashMap::new();
    for file in all_files {
        let raw_markdown_file = read_file_contents(&file.content);
        let parsed_markdown_file = parse_markdown_file(raw_markdown_file);
        parsed_file_index.insert(page_name(file.name.clone()), parsed_markdown_file);
    }
    parsed_file_index
}

pub fn remove_file_from_page_index(
    page_index: &UserPageIndex,
    simple_page_name: &SimplePageName,
) -> UserPageIndex {
    UserPageIndex {
        entries: remove_from_index(&page_index.entries, simple_page_name),
    }
}

pub fn remove_file_from_journal_index(
    page_index: &JournalPageIndex,
    simple_page_name: &SimplePageName,
) -> JournalPageIndex {
    JournalPageIndex {
        entries: remove_from_index(&page_index.entries, simple_page_name),
    }
}

fn remove_from_index(
    current_index: &HashMap<SimplePageName, ParsedMarkdownFile>,
    page_name: &SimplePageName,
) -> HashMap<SimplePageName, ParsedMarkdownFile> {
    let mut result = HashMap::new();
    for key in current_index.keys() {
        if key != page_name {
            result.insert(key.clone(), current_index.get(key).unwrap().clone());
        }
    }
    result
}
