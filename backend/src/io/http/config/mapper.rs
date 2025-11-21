use crate::io::http::config::dtos::JournalConfigurationDto;
use crate::looksyk::data::config::runtime_graph_configuration::{
    JournalConfigration, JournalTitleFormat, ShowWeekdayInTitle,
};
use std::str::FromStr;

impl From<&JournalConfigration> for JournalConfigurationDto {
    fn from(val: &JournalConfigration) -> Self {
        JournalConfigurationDto {
            journal_title_format: val.journal_title_format.to_string(),
            show_weekday_in_title: val.show_weekday_in_title.to_string(),
        }
    }
}

impl From<JournalConfigurationDto> for JournalConfigration {
    fn from(val: JournalConfigurationDto) -> Self {
        JournalConfigration {
            journal_title_format: JournalTitleFormat::from_str(&val.journal_title_format)
                .unwrap_or(JournalTitleFormat::World),
            show_weekday_in_title: ShowWeekdayInTitle::from_str(&val.show_weekday_in_title)
                .unwrap_or(ShowWeekdayInTitle::None),
        }
    }
}
