use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JournalPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}
