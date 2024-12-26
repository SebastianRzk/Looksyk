use crate::looksyk::model::PageType;

pub const REL_MEDIA_CONFIG_PATH: &str = "config/media.json";
pub const REL_CONFIG_PATH: &str = "config/config.json";
pub const REL_USER_PAGE_LOCATION: &str = "pages/";
pub const REL_JOURNAL_PAGE_LOCATION: &str = "journals/";
pub const REL_MEDIA_LOCATION: &str = "assets/";

pub fn path_for_page_type(page_type: &PageType) -> &'static str {
    match page_type {
        PageType::UserPage => REL_USER_PAGE_LOCATION,
        PageType::JournalPage => REL_JOURNAL_PAGE_LOCATION,
    }
}
