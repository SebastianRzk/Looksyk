use std::collections::HashMap;
use std::io::Error;

use crate::looksyk::builder::page_name;
use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::queries::args::{
    parse_property, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_TAG,
};
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::state::todo::{TodoIndex, TodoState};

pub const QUERY_NAME_TODO_PROGRESS: &str = "todo-progress";

pub fn parse_query_todo_progress(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str
        .strip_prefix(QUERY_NAME_TODO_PROGRESS)
        .ok_or(Error::other(ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?
        .trim();
    let query_root_opt = parse_property(query_content, PARAM_TAG)?;

    let mut args1 = HashMap::new();
    args1.insert(PARAM_TAG.to_string(), query_root_opt.value);
    Ok(Query {
        query_type: QueryType::TodoProgress,
        display: QueryDisplayType::Unknown,
        args: args1,
    })
}

pub fn render_todo_query_progress(query: Query, data: &TodoIndex) -> QueryRenderResult {
    let expected_tag = query.args.get(PARAM_TAG).unwrap();
    let expected_tag_page = page_name(expected_tag.clone());

    let mut todo_count = 0;
    let mut done_count = 0;

    for todo in &data.entries {
        if todo.tags.contains(&expected_tag_page) {
            todo_count += 1;
            if todo.state == TodoState::Done {
                done_count += 1;
            }
        }
    }

    render_progress(expected_tag, todo_count, done_count)
}

fn render_progress(tag_name: &str, todo_count: u32, done_count: u32) -> QueryRenderResult {
    let percent_done = if todo_count > 0 {
        (done_count as f64 / todo_count as f64 * 100.0).round() as u32
    } else {
        100
    };
    QueryRenderResult {
        has_dynamic_content: true,
        referenced_markdown: vec![],
        inplace_markdown: format!(
            "<label>
{tag_name} -Todos : {done_count}/{todo_count} done ({percent_done}%)
 <progress value=\"{percent_done}\" max=\"100\"></progress>
</label>"
        ),
    }
}
