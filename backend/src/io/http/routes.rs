use crate::io::http::link::encode_link_component;
use crate::looksyk::model::SimplePageName;

pub fn to_wiki_page_url(page_name: &SimplePageName) -> String {
    format!(
        "/page/{}",
        encode_link_component(&page_name.name)
    )
}

pub fn to_journal_page_url(page_name: &SimplePageName) -> String {
    format!(
        "/journal/{}", &page_name.name
    )
}