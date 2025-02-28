use crate::looksyk::model::QueryRenderResult;

pub fn render_as_count<T>(refs: &[T]) -> QueryRenderResult {
    QueryRenderResult {
        inplace_markdown: refs.len().to_string(),
        referenced_markdown: vec![],
        has_dynamic_content: false,
    }
}
