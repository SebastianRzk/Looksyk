use serde::Serialize;

#[derive(Serialize)]
pub struct MetaInfoDto {
    pub tags: Vec<String>,
    pub media: Vec<String>,
}

#[derive(Serialize)]
pub struct TitleDto {
    pub title: String,
}
