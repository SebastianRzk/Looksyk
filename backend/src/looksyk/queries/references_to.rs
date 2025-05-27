use std::collections::{HashMap, HashSet};
use std::io::Error;

use crate::looksyk::model::{PageId, QueryRenderResult, SimplePageName};
use crate::looksyk::queries::args::{
    parse_display_type, parse_property, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_TARGET,
};
use crate::looksyk::queries::basic::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::renderer::{render_link_by_id, render_user_link};
use crate::state::tag::TagIndex;

pub const QUERY_NAME_REFERENCES_TO: &str = "references-to";

pub fn parse_query_references_to(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str
        .strip_prefix(QUERY_NAME_REFERENCES_TO)
        .ok_or(Error::other(ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?
        .trim();
    let query_target_opt = parse_property(query_content, PARAM_TARGET)?;

    let display_type = parse_display_type(query_target_opt.remaining_text.clone())?;

    let mut args1 = HashMap::new();
    args1.insert(PARAM_TARGET.to_string(), query_target_opt.value);
    let (display_type, args) = (display_type, args1);
    Ok(Query {
        query_type: QueryType::ReferencesTo,
        display: display_type,
        args,
    })
}

pub fn render_references_of_query(query: Query, data: &TagIndex) -> QueryRenderResult {
    let target = SimplePageName {
        name: query.args.get(PARAM_TARGET).unwrap().clone(),
    };

    let empty_set: HashSet<PageId> = HashSet::new();
    let references = data
        .entries
        .get(&target.as_user_page())
        .unwrap_or(&empty_set);

    match query.display {
        QueryDisplayType::InplaceList => render_as_list(&target, references),
        QueryDisplayType::Count => render_as_count(references),
        _ => render_display_unknown(
            query.display,
            vec![QueryDisplayType::InplaceList, QueryDisplayType::Count],
        ),
    }
}

pub fn render_as_count(refs: &HashSet<PageId>) -> QueryRenderResult {
    QueryRenderResult {
        inplace_markdown: refs.len().to_string(),
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

fn render_as_list(page_name: &SimplePageName, refs: &HashSet<PageId>) -> QueryRenderResult {
    let mut result = format!("Pages that reference {}\n", render_user_link(page_name));
    for r in refs.iter() {
        result.push_str(format!("* {}\n", render_link_by_id(r)).as_str());
    }
    if refs.is_empty() {
        result.push_str("* No references found!\n");
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
        has_dynamic_content: false,
    }
}
