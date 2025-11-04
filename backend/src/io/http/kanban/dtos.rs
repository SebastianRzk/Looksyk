use crate::io::http::page::dtos::PreparedReferencedMarkdownDto;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct MoveKanbanItemRequestDto {
    pub reference: ReferenceDto,
    pub key: String,
    pub from: String,
    pub to: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceDto {
    pub file_id: String,
    pub block_number: usize,
}
