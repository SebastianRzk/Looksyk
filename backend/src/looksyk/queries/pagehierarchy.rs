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
    Ok(Query {
        query_type: QueryType::PageHierarchy,
        display: display_type,
        args,
    })
}


pub fn render_page_hierarchy(query: Query, data: &UserPageIndex) -> QueryRenderResult {
    let root = query.args.get(PARAM_ROOT).unwrap();
    let keys: Vec<&SimplePageName> = data.entries.keys().into_iter().collect();

    let mut result = filter_pages_by_root(root, keys);
    result.sort_unstable_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    match query.display {
        QueryDisplayType::InplaceList => render_as_list(root, result),
        QueryDisplayType::Count => render_as_count(result),
        _ => render_display_unknown(query.display)
    }
}

fn filter_pages_by_root<'a>(root: &String, keys: Vec<&'a SimplePageName>) -> Vec<&'a SimplePageName> {
    let mut result = vec![];

    for page in keys {
        if page.name.starts_with(root) {
            result.push(page);
        }
    }
    result
}

pub fn render_as_count(selected_pages: Vec<&SimplePageName>) -> QueryRenderResult {
    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: selected_pages.len().to_string(),
        has_dynamic_content: false,
    }
}

pub fn render_as_list(root_name: &String, selected_pages: Vec<&SimplePageName>) -> QueryRenderResult {
    let mut result = format!("{}:\n", root_name);
    for page in selected_pages {
        result.push_str("- ");
        result.push_str(render_user_link(&page).as_str());
        result.push_str("\n")
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
        has_dynamic_content: false,
    }
}

#[cfg(test)]
mod test {
    use crate::looksyk::model::SimplePageName;
    use crate::looksyk::query::{QueryDisplayType, QueryType};

    #[test]
    fn test_parse_query_page_hierarchy() {
        let query = "page-hierarchy root:\"foo\" display:\"inplace-list\"";
        let result = super::parse_query_page_hierarchy(query).unwrap();
        assert_eq!(result.query_type, QueryType::PageHierarchy);
        assert_eq!(result.display, QueryDisplayType::InplaceList);
        assert_eq!(result.args.get("root").unwrap(), "foo");
    }

    #[test]
    fn test_filter_pages(){
        let root = "foo".to_string();

        let page_foo = SimplePageName { name: "foo".to_string() };
        let page_foo_bar = SimplePageName { name: "foo/bar".to_string() };
        let page_foo_bar_baz = SimplePageName { name: "foo/bar/baz".to_string() };
        let page_bar = SimplePageName { name: "bar".to_string() };
        let page_bar_baz = SimplePageName { name: "bar/foo".to_string() };

        let mut keys = vec![
            &page_foo,
            &page_foo_bar,
            &page_foo_bar_baz,
            &page_bar,
            &page_bar_baz,
        ];
        let result = super::filter_pages_by_root(&root, keys);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "foo");
        assert_eq!(result[1].name, "foo/bar");
        assert_eq!(result[2].name, "foo/bar/baz");
    }
}