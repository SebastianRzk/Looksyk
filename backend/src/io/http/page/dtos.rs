use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedBlockContentDto {
    pub original_text: String,
    pub prepared_markdown: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedBlockDto {
    pub indentation: usize,
    pub content: PreparedBlockContentDto,
    pub referenced_content: Vec<PreparedReferencedMarkdownDto>,
    pub has_dynamic_content: bool,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownReferenceDto {
    pub file_id: String,
    pub file_name: String,
    pub block_number: usize,
    pub link: String,
}

#[derive(Serialize)]
pub struct PreparedReferencedMarkdownDto {
    pub content: PreparedBlockContentDto,
    pub reference: MarkdownReferenceDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedMarkdownFileDto {
    pub is_favourite: bool,
    pub blocks: Vec<PreparedBlockDto>,
}

#[derive(Deserialize)]
pub struct ToValidate {
    pub block: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockDto {
    pub indentation: usize,
    pub markdown: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockContentDto {
    pub markdown: String,
}

#[derive(Deserialize)]
pub struct UpdateMarkdownFileDto {
    pub blocks: Vec<UpdateBlockDto>,
}
