use std::collections::HashMap;
use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};

#[derive(Clone)]
pub struct JournalPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}
