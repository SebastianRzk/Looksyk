use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::queries::args::ERROR_DISPLAY_TYPE_UNKNOWN;
use crate::looksyk::query::QueryDisplayType;

pub fn render_display_unknown(display_type: QueryDisplayType) -> QueryRenderResult {
    if display_type == QueryDisplayType::Unknown {
        return QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: ERROR_DISPLAY_TYPE_UNKNOWN.to_string(),
            referenced_markdown: vec![],
        };
    }
    QueryRenderResult {
        has_dynamic_content: false,
        inplace_markdown: format!("display type {} not suppoerted for querytype", display_type.to_string()),
        referenced_markdown: vec![],
    }
}