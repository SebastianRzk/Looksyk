use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JournalPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::model::ParsedMarkdownFile;
    use crate::state::journal::JournalPageIndex;

    pub fn journal_page_index(name: &str, entry: ParsedMarkdownFile) -> JournalPageIndex {
        let mut entries = std::collections::HashMap::new();
        entries.insert(
            crate::looksyk::builder::page_name_str(name),
            entry,
        );
        JournalPageIndex { entries }
    }
}
