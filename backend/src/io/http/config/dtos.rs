use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalConfigurationDto {
    pub journal_title_format: String,
    pub show_weekday_in_title: String,
}
