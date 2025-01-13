use crate::looksyk::queries::insert_file_content::QUERY_NAME_INSERT_FILE_CONTENT;
use crate::looksyk::queries::pagehierarchy::QUERY_NAME_PAGE_HIERARCHY;
use crate::looksyk::queries::references_to::QUERY_NAME_REFERENCES_TO;
use crate::looksyk::queries::todo::QUERY_NAME_TODOS;

pub fn available_query_types() -> String {
    format!(
        "{}, {}, {}, {}",
        QUERY_NAME_PAGE_HIERARCHY,
        QUERY_NAME_REFERENCES_TO,
        QUERY_NAME_TODOS,
        QUERY_NAME_INSERT_FILE_CONTENT
    )
}
