use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::queries::args::{
    parse_display_type, ParamBuilder, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX,
};
use crate::looksyk::queries::basic::extra_param_validation::ParamValidator;
use crate::looksyk::queries::basic::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::syntax::markdown::render_as_image;
use std::io::Error;

pub const QUERY_NAME_PLOT_PROPERTY: &str = "plot-property";

pub const PARAM_LABEL: &str = "label";

pub const PARAM_CAPTION: &str = "caption";

pub const PARAM_WIDTH: &str = "width";

pub const PARAM_HEIGHT: &str = "height";

pub const PARAM_PROPERTY_KEY: &str = "propertyKey";

pub const PARAM_STARTING_AT: &str = "startingAt";

pub const PARAM_ENDING_AT: &str = "endingAt";

pub fn parse_query_plot_property(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str
        .strip_prefix(QUERY_NAME_PLOT_PROPERTY)
        .ok_or(Error::other(ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?
        .trim();

    let parser_result = ParamBuilder::init(query_content.to_string())
        .next(PARAM_PROPERTY_KEY)?
        .next(PARAM_LABEL)?
        .next(PARAM_CAPTION)?
        .next(PARAM_WIDTH)?
        .next(PARAM_HEIGHT)?
        .next(PARAM_STARTING_AT)?
        .next(PARAM_ENDING_AT)?
        .build();

    let display_type = parse_display_type(parser_result.remaining_value)?;

    Ok(Query {
        query_type: QueryType::PlotProperty,
        display: display_type,
        args: parser_result.parsed_args,
    })
}

pub fn render_plot_property_query(query: Query) -> QueryRenderResult {
    match query.display {
        QueryDisplayType::Linechart => render_plot_property_as_linechart(query),
        _ => render_display_unknown(query.display, vec![QueryDisplayType::Linechart]),
    }
}

fn render_plot_property_as_linechart(query: Query) -> QueryRenderResult {
    let extra_validation = ParamValidator::default()
        .validate_as_integer(query.get_arg("width").unwrap(), PARAM_WIDTH)
        .validate_as_integer(query.get_arg("height").unwrap(), PARAM_HEIGHT)
        .validate_as_date(query.get_arg("startingAt").unwrap(), PARAM_STARTING_AT)
        .validate_as_date(query.get_arg("endingAt").unwrap(), PARAM_ENDING_AT);

    if extra_validation.has_errors() {
        return QueryRenderResult {
            inplace_markdown: extra_validation.format_errors_as_markdown(),
            referenced_markdown: vec![],
            has_dynamic_content: false,
        };
    }

    let img_source = format!(
        "/api/plot/?label={}&propertyKey={}&caption={}&width={}&height={}&startingAt={}&endingAt={}",
        query.args.get(PARAM_LABEL).unwrap(),
        query.args.get(PARAM_PROPERTY_KEY).unwrap(),
        query.args.get(PARAM_CAPTION).unwrap(),
        query.args.get(PARAM_WIDTH).unwrap(),
        query.args.get(PARAM_HEIGHT).unwrap(),
        query.args.get(PARAM_STARTING_AT).unwrap(),
        query.args.get(PARAM_ENDING_AT).unwrap(),
    );

    let img_markdown = format!(
        "<img alt=\"{}\" src=\"{}\"/>",
        query.args.get(PARAM_LABEL).unwrap(),
        &img_source
    );

    QueryRenderResult {
        inplace_markdown: img_markdown,
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::queries::plot::{
        render_plot_property_query, PARAM_CAPTION, PARAM_ENDING_AT, PARAM_HEIGHT, PARAM_LABEL,
        PARAM_PROPERTY_KEY, PARAM_STARTING_AT, PARAM_WIDTH,
    };
    use crate::looksyk::query::{Query, QueryDisplayType, QueryType};

    #[test]
    fn test_parse_query() {
        let query = "plot-property propertyKey:\"status\" label:\"MyPlot\" caption:\"This is my plot\" width:\"800\" height:\"600\" startingAt:\"2023-01-01\" endingAt:\"2023-12-31\" display:\"linechart\"";

        let result = super::parse_query_plot_property(query).unwrap();

        assert_eq!(result.query_type, QueryType::PlotProperty);
        assert_eq!(result.display, QueryDisplayType::Linechart);
        assert_eq!(result.args.get(PARAM_LABEL).unwrap(), "MyPlot");
        assert_eq!(result.args.get(PARAM_CAPTION).unwrap(), "This is my plot");
        assert_eq!(result.args.get(PARAM_WIDTH).unwrap(), "800");
        assert_eq!(result.args.get(PARAM_HEIGHT).unwrap(), "600");
        assert_eq!(result.args.get(PARAM_PROPERTY_KEY).unwrap(), "status");
        assert_eq!(result.args.get(PARAM_STARTING_AT).unwrap(), "2023-01-01");
        assert_eq!(result.args.get(PARAM_ENDING_AT).unwrap(), "2023-12-31");
    }

    #[test]
    fn test_render_plot_as_linechart() {
        let result = render_plot_property_query(Query {
            query_type: QueryType::PlotProperty,
            display: QueryDisplayType::Linechart,
            args: vec![
                (PARAM_LABEL.to_string(), "BoardBoardBoard".to_string()),
                (PARAM_PROPERTY_KEY.to_string(), "my-board".to_string()),
                (PARAM_WIDTH.to_string(), "800".to_string()),
                (PARAM_HEIGHT.to_string(), "600".to_string()),
                (
                    PARAM_CAPTION.to_string(),
                    "This is my kanban board".to_string(),
                ),
                (PARAM_STARTING_AT.to_string(), "2023-01-01".to_string()),
                (PARAM_ENDING_AT.to_string(), "2023-12-31".to_string()),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        });
        assert_eq!(
            result.inplace_markdown,
            "<img src=\"/api/plot/?label=BoardBoardBoard&propertyKey=my-board&caption=This is my kanban board&width=800&height=600&startingAt=2023-01-01&endingAt=2023-12-31\"/>"
        );
        assert_eq!(result.has_dynamic_content, false);
        assert_eq!(result.referenced_markdown.len(), 0);
    }

    #[test]
    fn test_render_plot_as_linechart_with_invalid_params() {
        let result = render_plot_property_query(Query {
            query_type: QueryType::PlotProperty,
            display: QueryDisplayType::Linechart,
            args: vec![
                (PARAM_LABEL.to_string(), "BoardBoardBoard".to_string()),
                (PARAM_PROPERTY_KEY.to_string(), "my-board".to_string()),
                (PARAM_WIDTH.to_string(), "invalid_width".to_string()),
                (PARAM_HEIGHT.to_string(), "invalid_height".to_string()),
                (
                    PARAM_CAPTION.to_string(),
                    "This is my kanban board".to_string(),
                ),
                (
                    PARAM_STARTING_AT.to_string(),
                    "invalid_starting_date".to_string(),
                ),
                (
                    PARAM_ENDING_AT.to_string(),
                    "invalid_ending_date".to_string(),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        });
        assert_eq!(
            result.inplace_markdown,
            "**Parameter Validation Errors:**\n- Parameter 'width' with value 'invalid_width' is not a valid integer.\n- Parameter 'height' with value 'invalid_height' is not a valid integer.\n- Parameter 'startingAt' with value 'invalid_starting_date' is not a valid date (expected format: YYYY-MM-DD).\n- Parameter 'endingAt' with value 'invalid_ending_date' is not a valid date (expected format: YYYY-MM-DD).\n"
        );
        assert_eq!(result.has_dynamic_content, false);
        assert_eq!(result.referenced_markdown.len(), 0);
    }
}
