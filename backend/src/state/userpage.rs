use std::collections::HashMap;
use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};

#[derive(Clone)]
pub struct UserPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}