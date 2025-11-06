use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::queries::args::{
    parse_display_type, ParamBuilder, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_TAG,
};
use crate::looksyk::queries::basic::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use std::io::Error;

pub const PARAM_TITLE: &str = "title";

pub const QUERY_NAME_BOARD: &str = "board";

pub const PARAM_QUERY_COLUMN_KEY: &str = "columnKey";

pub const PARAM_QUERY_COLUMN_VALUES: &str = "columnValues";

pub const PARAM_QUERY_PRIORITY_KEY: &str = "priorityKey";

pub fn parse_query_board(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str
        .strip_prefix(QUERY_NAME_BOARD)
        .ok_or(Error::other(ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?
        .trim();

    let parser_result = ParamBuilder::init(query_content.to_string())
        .next(PARAM_TITLE)?
        .next(PARAM_TAG)?
        .next(PARAM_QUERY_COLUMN_KEY)?
        .next(PARAM_QUERY_COLUMN_VALUES)?
        .next(PARAM_QUERY_PRIORITY_KEY)?
        .build();

    let display_type = parse_display_type(parser_result.remaining_value)?;

    Ok(Query {
        query_type: QueryType::Board,
        display: display_type,
        args: parser_result.parsed_args,
    })
}

pub fn render_board_query(query: Query) -> QueryRenderResult {
    match query.display {
        QueryDisplayType::Link => render_kanban_as_link(query),
        _ => render_display_unknown(query.display, vec![QueryDisplayType::Link]),
    }
}

fn render_kanban_as_link(query: Query) -> QueryRenderResult {
    let link_data = format!("{{\"title\":\"{}\",\"tag\":\"{}\",\"columnKey\":\"{}\",\"columnValues\":[{}],\"priorityKey\":\"{}\"}}",
                            query.args.get(PARAM_TITLE).unwrap(),
                            query.args.get(PARAM_TAG).unwrap(),
                            query.args.get(PARAM_QUERY_COLUMN_KEY).unwrap(),
                            prepare_column_values_parameter(&query),
                            query.args.get(PARAM_QUERY_PRIORITY_KEY).unwrap());
    let urlencoded_link_data = urlencoding::encode(&link_data);

    let link = format!(
        "[{}](/special-page/kanban?data={})",
        query.args.get(PARAM_TITLE).unwrap(),
        urlencoded_link_data,
    );

    QueryRenderResult {
        inplace_markdown: link,
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

fn prepare_column_values_parameter(query: &Query) -> String {
    let mut arg = query
        .args
        .get(PARAM_QUERY_COLUMN_VALUES)
        .unwrap()
        .to_string();
    arg = arg
        .trim_start_matches(",")
        .trim_end_matches(",")
        .to_string();

    if !arg.contains(",") {
        return format!("\"{}\"", arg);
    }
    arg.split(",")
        .map(|x| format!("\"{}\"", x))
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use crate::looksyk::queries::args::PARAM_TAG;
    use crate::looksyk::queries::kanban::{
        render_board_query, PARAM_QUERY_COLUMN_KEY, PARAM_QUERY_COLUMN_VALUES,
        PARAM_QUERY_PRIORITY_KEY, PARAM_TITLE,
    };
    use crate::looksyk::query::{Query, QueryDisplayType, QueryType};

    #[test]
    fn test_parse_query() {
        let query = "board title:\"My first Kanban\" tag:\"kanban\" columnKey:\"state\" columnValues:\"TODO,DOING,DONE\" priorityKey:\"priority\" display:\"link\"}";

        let result = super::parse_query_board(query).unwrap();

        assert_eq!(result.query_type, super::QueryType::Board);
        assert_eq!(result.display, super::QueryDisplayType::Link);
        assert_eq!(result.args.get(PARAM_TITLE).unwrap(), "My first Kanban");
        assert_eq!(result.args.get(PARAM_TAG).unwrap(), "kanban");
        assert_eq!(result.args.get(PARAM_QUERY_COLUMN_KEY).unwrap(), "state");
        assert_eq!(
            result.args.get(PARAM_QUERY_COLUMN_VALUES).unwrap(),
            "TODO,DOING,DONE"
        );
        assert_eq!(
            result.args.get(PARAM_QUERY_PRIORITY_KEY).unwrap(),
            "priority"
        );
    }

    #[test]
    fn test_render_boards_query_as_link() {
        let result = render_board_query(Query {
            query_type: QueryType::Board,
            display: QueryDisplayType::Link,
            args: vec![
                (PARAM_TITLE, "BoardBoardBoard"),
                (PARAM_TAG, "my-board"),
                (PARAM_QUERY_COLUMN_KEY, "status"),
                (PARAM_QUERY_COLUMN_VALUES, "TODO,DOING,DONE,BLOCKED"),
                (PARAM_QUERY_PRIORITY_KEY, "prio"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        });
        assert_eq!(
            result.inplace_markdown,
            "[BoardBoardBoard](/special-page/kanban?data=%7B%22title%22%3A%22BoardBoardBoard%22%2C%22tag%22%3A%22my-board%22%2C%22columnKey%22%3A%22status%22%2C%22columnValues%22%3A%5B%22TODO%22%2C%22DOING%22%2C%22DONE%22%2C%22BLOCKED%22%5D%2C%22priorityKey%22%3A%22prio%22%7D)"
        );
        assert_eq!(result.has_dynamic_content, false);
        assert_eq!(result.referenced_markdown.len(), 0);
    }
}
