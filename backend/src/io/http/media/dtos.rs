use serde::Serialize;

#[derive(Serialize)]
pub struct SuggestionsDto {
    pub suggestions: Vec<SuggestionDto>,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionDto {
    pub explanation: String,
    pub inplace_markdown: String,
}