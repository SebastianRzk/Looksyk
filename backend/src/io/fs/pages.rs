use std::fs;
use std::path::Path;

use crate::io::fs::basic_file::read_file;
use crate::io::fs::paths::path_for_page_type;
use crate::looksyk::model::PageType;
use crate::state::DataRootLocation;

pub fn read_all_user_files(data_root_location: &DataRootLocation) -> Vec<PageOnDisk> {
    let page_path = Path::new(data_root_location.path.as_str()).join(path_for_page_type(&PageType::UserPage));
    return read_all_files(page_path.to_str().unwrap());
}

pub fn read_all_journal_files(data_root_location: &DataRootLocation) -> Vec<PageOnDisk> {
    let journal_path = Path::new(data_root_location.path.as_str()).join(path_for_page_type(&PageType::JournalPage));
    return read_all_files(journal_path.to_str().unwrap());
}


fn read_all_files(data_path: &str) -> Vec<PageOnDisk> {
    println!("Reading {}", data_path);
    let directory_list = fs::read_dir(data_path).unwrap();
    let mut all_files = vec![];

    for file in directory_list {
        let dir = file.unwrap();
        let path = dir.path();
        let file_content = read_file(path.clone());
        let file_stem = Path::file_stem(path.as_path()).unwrap().to_str().unwrap();
        all_files.push(PageOnDisk {
            name: file_stem.to_string(),
            content: file_content,
        });
    }

    all_files
}


pub fn write_page(page: PageOnDisk, data_path: &DataRootLocation, page_type: &PageType) {
    let page_name = page.name;
    let encoded_page_name = page_name.replace("/", "%2F");
    let destination = Path::new(data_path.path.as_str()).join(path_for_page_type(page_type)).join(encoded_page_name + ".md");
    println!("writing to {}", destination.to_str().unwrap());
    let content_with_newline = format!("{}\n", page.content);
    fs::write(destination, content_with_newline).unwrap();
}

pub struct PageOnDisk {
    pub name: String,
    pub content: String,
}