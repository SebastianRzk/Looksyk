use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::looksyk::builder::page_name;
use crate::looksyk::model::{MarkdownReference, QueryRenderResult, ReferencedMarkdown};
use crate::looksyk::queries::args::{parse_display_type, parse_property};
use crate::looksyk::queries::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::renderer::{render_block_flat, render_user_link};
use crate::state::{TodoIndex, TodoIndexEntry, TodoState};

pub fn parse_query_todo(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str.strip_prefix("todos").ok_or(Error::new(ErrorKind::Other, "Decode error"))?.trim();
    let query_root_opt = parse_property(query_content, "tag")?;
    let query_state_opt = parse_property(query_root_opt.remaining_text.as_str(), "state")?;
    let display_type1 = parse_display_type(query_state_opt.remaining_text.clone())?;

    let mut args1 = HashMap::new();
    args1.insert("tag".to_string(), query_root_opt.value);
    args1.insert("state".to_string(), query_state_opt.value);
    let (display_type, args) = (display_type1, args1);
    return Ok(Query {
        query_type: QueryType::Todo,
        display: display_type,
        args,
    });
}


pub fn render_todo_query(query: Query, data: &TodoIndex) -> QueryRenderResult {
    let expected_tag = query.args.get("tag").unwrap();
    let expected_tag_page = page_name(expected_tag.clone());
    let expected_state = state_from_string(query.args.get("state").unwrap());


    let mut result = vec![];


    for todo in &data.entries {
        if todo.state == expected_state {
            if todo.tags.contains(&expected_tag_page) {
                result.push(todo);
            }
        }
    }


    match query.display {
        QueryDisplayType::InplaceList => render_as_list(result),
        QueryDisplayType::Count => render_as_count(result),
        QueryDisplayType::ReferencedList => render_as_references(result),
        _ => render_display_unknown(query.display)
    }
}


fn state_from_string(state: &String) -> TodoState {
    if state.to_lowercase() == "done" {
        return TodoState::Done;
    }
    TodoState::Todo
}


fn render_as_references(selected_todos: Vec<&TodoIndexEntry>) -> QueryRenderResult {
    QueryRenderResult {
        inplace_markdown: "".to_string(),
        referenced_markdown: selected_todos.iter().map(|x| {
            ReferencedMarkdown {
                content: x.block.clone(),
                refernce: MarkdownReference {
                    page_id: x.source.page_id.clone(),
                    block_number: x.source.blocknumber,
                    page_name: x.source.page_name.clone(),
                },
            }
        }).collect(),
    }
}


fn render_as_count(selected_todos: Vec<&TodoIndexEntry>) -> QueryRenderResult {
    return QueryRenderResult {
        inplace_markdown: selected_todos.len().to_string(),
        referenced_markdown: vec![],
    };
}

fn render_as_list(selected_selected_todos: Vec<&TodoIndexEntry>) -> QueryRenderResult {
    let mut result = "".to_string();
    for todo in selected_selected_todos {
        if todo.state == TodoState::Done {
            result.push_str("* ☑ ");
        } else {
            result.push_str("*🔲 ☐ ");
        }
        result.push_str(render_user_link(&todo.source.page_name).as_str());
        result.push_str(": ");
        result.push_str(render_block_flat(&todo.block).as_str());
        result.push_str("\n")
    }
    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
    }
}