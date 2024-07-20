use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::query::QueryDisplayType;

pub fn render_display_unknown(display_type: QueryDisplayType) -> QueryRenderResult {
    if display_type == QueryDisplayType::Unknown {
        return QueryRenderResult {
            inplace_markdown: "display type unknown".to_string(),
            referenced_markdown: vec![],
        };
    }
    QueryRenderResult {
        inplace_markdown: format!("display type {} not suppoerted for querytype", display_type.to_string()),
        referenced_markdown: vec![],
    }
}