use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use crate::looksyk::config::config::Config;
use crate::looksyk::index::media::MediaIndex;
use crate::looksyk::model::{PageId, ParsedBlock, ParsedMarkdownFile, SimplePageName};

pub struct AppState {
    pub media_index: Mutex<MediaIndex>,
    pub data_path: DataRootLocation,
    pub user_pages: Mutex<UserPageIndex>,
    pub journal_pages: Mutex<JournalPageIndex>,
    pub todo_index: Mutex<TodoIndex>,
    pub tag_index: Mutex<TagIndex>,
    pub config: Mutex<Config>,
}

pub struct DataRootLocation{
    pub path: String,
}


#[derive(Clone)]
pub struct UserPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}

#[derive(Clone)]
pub struct JournalPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}


#[derive(Clone)]
pub struct TodoIndex {
    pub entries: Vec<TodoIndexEntry>,
}


#[derive(Clone)]
pub struct TodoIndexEntry {
    pub state: TodoState,
    pub source: TodoSourceReference,
    pub block: ParsedBlock,
    pub tags: Vec<SimplePageName>,
}


#[derive(Clone)]
pub struct TagIndex {
    pub entries: HashMap<PageId, HashSet<PageId>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TodoSourceReference {
    pub page_id: PageId,
    pub page_name: SimplePageName,
    pub blocknumber: usize,
}


#[derive(Clone, PartialEq, Debug)]
pub enum TodoState {
    Todo,
    Done,
}
