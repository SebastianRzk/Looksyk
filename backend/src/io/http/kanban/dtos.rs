use serde::{Deserialize, Serialize};
use crate::io::http::page::dtos::PreparedReferencedMarkdownDto;

#[derive(Serialize)]
pub struct KanbanDataDto {
    pub title: String,
    pub lists: Vec<KanbanListDto>,
}

#[derive(Serialize)]
pub struct KanbanListDto {
    pub title: String,
    pub items: Vec<KanbanItemDto>,
}

#[derive(Serialize)]
pub struct KanbanItemDto {
    pub block: PreparedReferencedMarkdownDto,
    pub priority: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKanbanRequestDto {
    pub title: String,
    pub tag: String,
    pub column_identifier: String,
    pub column_values: Vec<String>,
    pub priority_identifier: String,
}