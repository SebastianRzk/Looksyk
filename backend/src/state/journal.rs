use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
use std::collections::HashMap;

#[derive(Clone)]
pub struct JournalPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}


impl JournalPageIndex {
    pub fn find(&self, page_name: &SimplePageName) -> Option<&ParsedMarkdownFile> {
        self.entries.get(page_name)
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = (&SimplePageName, &ParsedMarkdownFile)> {
        self.entries.iter()
    }

    pub fn insert(&mut self, page_name: SimplePageName, entry: ParsedMarkdownFile) {
        self.entries.insert(page_name, entry);
    }
}

impl Default for JournalPageIndex {
    fn default() -> Self {
        JournalPageIndex {
            entries: HashMap::new(),
        }
    }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::model::ParsedMarkdownFile;
    use crate::state::journal::JournalPageIndex;
    use std::collections::HashMap;

    pub fn journal_page_index(name: &str, entry: ParsedMarkdownFile) -> JournalPageIndex {
        let mut entries = HashMap::new();
        entries.insert(page_name_str(name), entry);
        JournalPageIndex { entries }
    }
}
