use crate::looksyk::model::{PageId, PageType, ParsedMarkdownFile};
use crate::looksyk::model::PageType::JournalPage;
use crate::state::journal::JournalPageIndex;
use crate::state::userpage::UserPageIndex;

pub struct MarkdownFileIndex<'a> {
    pub journal_page_index: &'a JournalPageIndex,
    pub user_page_index: &'a UserPageIndex,
}

impl MarkdownFileIndex<'_> {
    pub fn resolve(&self, id: &PageId) -> Option<&ParsedMarkdownFile> {
        match id.page_type {
            JournalPage => {
                self.journal_page_index.find(&id.name)
            }
            PageType::UserPage => self.user_page_index.find(&id.name),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::looksyk::builder::test_builder::{
        any_parsed_markdown_file, empty_journal_index, journal_page_id, user_page_id,
    };
    use crate::state::journal::builder::journal_page_index;
    use crate::state::markdown_file::MarkdownFileIndex;
    use crate::state::userpage::builder::{empty_user_page_index, user_page_index};

    #[test]
    fn test_resolve_with_journal_file() {
        let journal_name_str = "test-journal-name";
        let index = MarkdownFileIndex {
            journal_page_index: &journal_page_index(journal_name_str, any_parsed_markdown_file()),
            user_page_index: &empty_user_page_index(),
        };
        let resolved = index.resolve(&journal_page_id(journal_name_str));

        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap(), &any_parsed_markdown_file());
    }

    #[test]
    fn test_resolve_with_no_file() {
        let index = MarkdownFileIndex {
            journal_page_index: &empty_journal_index(),
            user_page_index: &empty_user_page_index(),
        };
        let resolved = index.resolve(&journal_page_id("non-existent-name"));

        assert!(resolved.is_none());
    }

    #[test]
    fn test_resolve_with_user_file() {
        let user_name_str = "test-user-name";
        let index = MarkdownFileIndex {
            journal_page_index: &empty_journal_index(),
            user_page_index: &user_page_index(user_name_str, any_parsed_markdown_file()),
        };
        let resolved = index.resolve(&user_page_id(user_name_str));

        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap(), &any_parsed_markdown_file());
    }
}
