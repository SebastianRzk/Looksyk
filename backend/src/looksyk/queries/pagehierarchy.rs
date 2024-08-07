use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::looksyk::model::{QueryRenderResult, SimplePageName};
use crate::looksyk::queries::args::{ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_ROOT, parse_display_type_for_lists, parse_property};
use crate::looksyk::queries::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::renderer::render_user_link;
use crate::state::userpage::UserPageIndex;

pub const QUERY_NAME_PAGE_HIERARCHY: &str = "page-hierarchy";

pub fn parse_query_page_hierarchy(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str.strip_prefix(QUERY_NAME_PAGE_HIERARCHY).ok_or(Error::new(ErrorKind::Other, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?.trim();
    let query_root_opt = parse_property(query_content, PARAM_ROOT)?;
    let display_type = parse_display_type_for_lists(query_root_opt.remaining_text.clone())?;

    let mut args = HashMap::new();
    args.insert("root".to_string(), query_root_opt.value);
    return Ok(Query {
        query_type: QueryType::PageHierarchy,
        display: display_type,
        args,
    });
}


pub fn render_page_hierarchy(query: Query, data: &UserPageIndex) -> QueryRenderResult {
    let root = query.args.get(PARAM_ROOT).unwrap();
    let mut keys: Vec<&SimplePageName> = data.entries.keys().into_iter().collect();
    keys.sort_unstable_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut result = vec![];

    for page in keys {
        if page.name.starts_with(root) {
            result.push(page);
        }
    }

    match query.display {
        QueryDisplayType::InplaceList => render_as_list(root, result),
        QueryDisplayType::Count => render_as_count(result),
        _ => render_display_unknown(query.display)
    }
}


pub fn render_as_count(selected_pages: Vec<&SimplePageName>) -> QueryRenderResult {
    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: selected_pages.len().to_string(),
    }
}

pub fn render_as_list(root_name: &String, selected_pages: Vec<&SimplePageName>) -> QueryRenderResult {
    let mut result = format!("{}:\n", root_name);
    for page in selected_pages {
        result.push_str("- ");
        result.push_str(render_user_link(page).as_str());
        result.push_str("\n")
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
    }
}