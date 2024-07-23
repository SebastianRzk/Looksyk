use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};

use crate::looksyk::model::{PageId, QueryRenderResult, SimplePageName};
use crate::looksyk::page_index::append_user_page_prefix;
use crate::looksyk::queries::args::{parse_display_type, parse_property};
use crate::looksyk::queries::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::renderer::{render_link_by_id, render_user_link};
use crate::state::tag::TagIndex;

pub fn parse_query_references_to(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str.strip_prefix("references-to").ok_or(Error::new(ErrorKind::Other, "Decode error"))?.trim();
    let query_target_opt = parse_property(query_content, "target")?;

    let display_type = parse_display_type(query_target_opt.remaining_text.clone())?;

    let mut args1 = HashMap::new();
    args1.insert("target".to_string(), query_target_opt.value);
    let (display_type, args) = (display_type, args1);
    return Ok(Query {
        query_type: QueryType::ReferencesTo,
        display: display_type,
        args,
    });
}


pub fn render_references_of_query(query: Query, data: &TagIndex) -> QueryRenderResult {
    let target = SimplePageName {
        name: query.args.get("target").unwrap().clone()
    };

    let empty_set: HashSet<PageId> = HashSet::new();
    let references = data.entries.get(&append_user_page_prefix(&target)).unwrap_or(&empty_set);


    match query.display {
        QueryDisplayType::InplaceList => render_as_list(&target, &references),
        QueryDisplayType::Count => render_as_count(references),
        _ => render_display_unknown(query.display)
    }
}


pub fn render_as_count(refs: &HashSet<PageId>) -> QueryRenderResult {
    return QueryRenderResult {
        inplace_markdown: refs.len().to_string(),
        referenced_markdown: vec![],
    };
}

fn render_as_list(page_name: &SimplePageName, refs: &HashSet<PageId>) -> QueryRenderResult {
    let mut result = format!("Pages that reference {}\n", render_user_link(&page_name));
    for r in refs.iter() {
        result.push_str(format!("* {}\n", render_link_by_id(&r)).as_str());
    }
    if refs.is_empty() {
        result.push_str("* No references found!\n");
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
    }
}