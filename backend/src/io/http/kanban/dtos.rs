use serde::Serialize;
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

pub struct GetKanbanRequestDto {
    title: String,
    tag: String,
    column_identifier: String,
    column_values: Vec<String>,
    priority_identifier: String,
}