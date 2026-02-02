#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlotDataDto {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub property_key: String,
    pub starting_at: String,
    pub ending_at: String,
}
