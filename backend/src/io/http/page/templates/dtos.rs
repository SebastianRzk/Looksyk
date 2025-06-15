use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TemplatesDto {
    pub templates: Vec<TemplateDto>,
}

#[derive(Serialize)]
pub struct TemplateDto {
    pub title: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTemplateDto {
    pub template_id: String,
    pub page_id: String,
    pub block_number: usize,
}
