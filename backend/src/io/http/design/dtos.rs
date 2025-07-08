use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppearanceDto {
    pub appearance: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignConfigDto {
    pub primary_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub primary_shading: String,
    pub appearance: AppearanceDto,
}
