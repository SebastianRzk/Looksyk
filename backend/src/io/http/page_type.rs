use crate::io::http::link_encoding::{decode_link_component, encode_link_component};
use crate::looksyk::model::{PageId, PageType, SimplePageName};

pub const USER_PAGE_PREFIX: &str = "%%user-page/";
pub const JOURNAL_PAGE_PREFIX: &str = "%%journal-page/";

pub fn get_page_id_from_external_string(external_string: &str) -> PageId {
    if external_string.starts_with(USER_PAGE_PREFIX) {
        PageId {
            name: SimplePageName {
                name: decode_link_component(&external_string.replace(USER_PAGE_PREFIX, "")),
            },
            page_type: PageType::UserPage,
        }
    } else if external_string.starts_with(JOURNAL_PAGE_PREFIX) {
        PageId {
            name: SimplePageName {
                name: external_string.replace(JOURNAL_PAGE_PREFIX, ""),
            },
            page_type: PageType::JournalPage,
        }
    } else {
        panic!("Unknown page type")
    }
}

pub fn page_id_to_external_string(page_id: &PageId) -> String {
    match page_id.page_type {
        PageType::UserPage => {
            format!(
                "{}{}",
                USER_PAGE_PREFIX,
                encode_link_component(&page_id.name.name)
            )
        }
        PageType::JournalPage => {
            format!(
                "{}{}",
                JOURNAL_PAGE_PREFIX,
                encode_link_component(&page_id.name.name)
            )
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_page_id_from_external_string_with_user_page() {
        let page_id = super::get_page_id_from_external_string("%%user-page/test");
        assert_eq!(page_id.page_type, super::PageType::UserPage);
        assert_eq!(page_id.name.name, "test");
    }

    #[test]
    fn test_get_page_id_from_external_string_with_journal_page() {
        let page_id = super::get_page_id_from_external_string("%%journal-page/test");
        assert_eq!(page_id.page_type, super::PageType::JournalPage);
        assert_eq!(page_id.name.name, "test");
    }

    #[test]
    fn test_page_id_to_external_string_with_user_page() {
        let page_id = super::PageId {
            name: super::SimplePageName {
                name: "test".to_string(),
            },
            page_type: super::PageType::UserPage,
        };
        assert_eq!(
            super::page_id_to_external_string(&page_id),
            "%%user-page/test"
        );
    }

    #[test]
    fn test_page_id_to_external_string_with_journal_page() {
        let page_id = super::PageId {
            name: super::SimplePageName {
                name: "test".to_string(),
            },
            page_type: super::PageType::JournalPage,
        };
        assert_eq!(
            super::page_id_to_external_string(&page_id),
            "%%journal-page/test"
        );
    }
}
