use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::query::QueryDisplayType;

pub fn render_display_unknown(
    display_type: QueryDisplayType,
    vec: Vec<QueryDisplayType>,
) -> QueryRenderResult {
    let available_display_types = vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    QueryRenderResult {
        has_dynamic_content: false,
        inplace_markdown: format!(
            "display type {} not supported for querytype. Avaliable display types: {}",
            display_type.to_string(),
            available_display_types
        ),
        referenced_markdown: vec![],
    }
}
