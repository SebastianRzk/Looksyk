use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
use std::collections::HashMap;

#[derive(Clone)]
pub struct UserPageIndex {
    pub entries: HashMap<SimplePageName, ParsedMarkdownFile>,
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::model::{ParsedMarkdownFile, SimplePageName};
    use crate::state::userpage::UserPageIndex;
    use std::collections::HashMap;

    pub fn empty_user_page_index() -> UserPageIndex {
        user_page_index_with(HashMap::new())
    }

    pub fn user_page_index_with(
        content: HashMap<SimplePageName, ParsedMarkdownFile>,
    ) -> UserPageIndex {
        UserPageIndex { entries: content }
    }

    pub fn user_page_index(name: &str, entry: ParsedMarkdownFile) -> UserPageIndex {
        let mut entries = HashMap::new();
        entries.insert(page_name_str(name), entry);
        UserPageIndex { entries }
    }
}
