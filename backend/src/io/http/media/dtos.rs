use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::json::Json as MPJson;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub json: MPJson<Metadata>,
}



#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadResult {
    pub inline_markdown: String,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPreviewDto {
    pub markdown_preview: Option<String>,
    pub html_preview_link: Option<String>,
    pub properties: AssetPropertiesDto

}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPropertiesDto {
    pub size: String,
    pub full_qualified_path: String
}