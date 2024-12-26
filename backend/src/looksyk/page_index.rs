use crate::looksyk::model::{PageId, PageType, SimplePageName};

pub const USER_PAGE_PREFIX: &str = "%%user-page/";
pub const JOURNAL_PAGE_PREFIX: &str = "%%journal-page/";

pub fn append_user_page_prefix(page_name: &SimplePageName) -> PageId {
    PageId {
        id: format!("{}{}", USER_PAGE_PREFIX, page_name.name),
    }
}

pub fn append_page_prefix(page_name: &SimplePageName, page_type: &PageType) -> PageId {
    match page_type {
        PageType::UserPage => append_user_page_prefix(page_name),
        PageType::JournalPage => append_journal_page_prefix(page_name),
    }
}

pub fn append_journal_page_prefix(page_name: &SimplePageName) -> PageId {
    PageId {
        id: format!("{}{}", JOURNAL_PAGE_PREFIX, page_name.name),
    }
}

pub fn strip_user_page_prefix(page_id: &PageId) -> SimplePageName {
    SimplePageName {
        name: page_id.id.replace(USER_PAGE_PREFIX, ""),
    }
}

pub fn strip_journal_page_prefix(page_id: &PageId) -> SimplePageName {
    SimplePageName {
        name: page_id.id.replace(JOURNAL_PAGE_PREFIX, ""),
    }
}

pub fn strip_prefix(page_name: &PageId, page_type: &PageType) -> SimplePageName {
    match page_type {
        PageType::UserPage => strip_user_page_prefix(page_name),
        PageType::JournalPage => strip_journal_page_prefix(page_name),
    }
}

pub fn get_page_type(page_id: &PageId) -> PageType {
    if page_id.id.starts_with(USER_PAGE_PREFIX) {
        PageType::UserPage
    } else if page_id.id.starts_with(JOURNAL_PAGE_PREFIX) {
        PageType::JournalPage
    } else {
        panic!("Unknown page type")
    }
}
