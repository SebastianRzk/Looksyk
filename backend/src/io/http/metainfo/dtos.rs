use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MetaInfoDto {
    pub tags: Vec<String>,
    pub media: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TitleDto {
    pub title: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLocationDto {
    pub graph_location: String,
}
