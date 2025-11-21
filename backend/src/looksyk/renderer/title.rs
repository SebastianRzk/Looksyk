use crate::io::date::{calculate_journal_date_property, TodayContainer};
use crate::looksyk::data::config::runtime_graph_configuration::{
    JournalConfigration, JournalTitleFormat,
};
use crate::looksyk::model::{PageId, PageTitle, PageTitleSegment, SimplePageName};
use crate::looksyk::renderer::atomics::{journal_path, user_page_path};
use chrono::NaiveDate;

pub fn calculate_page_title(
    page_id: &PageId,
    journal_title_metadata: &JournalTitleCalculatorMetadata,
) -> PageTitle {
    match page_id.page_type {
        crate::looksyk::model::PageType::UserPage => calculate_user_page_title(page_id),
        crate::looksyk::model::PageType::JournalPage => {
            calculate_journal_page_title(page_id, journal_title_metadata)
        }
    }
}

pub fn calculate_user_page_title(page_id: &PageId) -> PageTitle {
    if page_id.name.name.contains("/") {
        let hierarchy_segments = page_id.name.name.split("/").collect::<Vec<&str>>();
        let mut title_segments: Vec<PageTitleSegment> = vec![];

        for i in 0..hierarchy_segments.len() {
            let segment_name = hierarchy_segments[i];
            let segment_full_name = hierarchy_segments[0..=i].join("/");
            title_segments.push(PageTitleSegment {
                title: segment_name.trim().to_string(),
                link: user_page_path(&SimplePageName {
                    name: segment_full_name.trim().to_string(),
                }),
            });
        }

        PageTitle {
            title: page_id.name.name.clone(),
            title_segments,
        }
    } else {
        PageTitle {
            title: page_id.name.name.clone(),
            title_segments: vec![PageTitleSegment {
                title: page_id.name.name.clone(),
                link: user_page_path(&page_id.name),
            }],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DateType {
    Today,
    Yesterday,
    Tomorrow,
    Other,
}

pub fn calculate_journal_page_title(
    page_id: &PageId,
    journal_title_metadata: &JournalTitleCalculatorMetadata,
) -> PageTitle {
    let splitted_date: Vec<&str> = page_id.name.name.split("_").collect();
    let year = splitted_date[0];
    let month = splitted_date[1];
    let day = splitted_date[2];
    let mut title = match journal_title_metadata
        .journal_configurataion
        .journal_title_format
    {
        JournalTitleFormat::World => format!("{day}.{month}.{year}"),
        JournalTitleFormat::American => format!("{month}/{day}/{year}"),
        JournalTitleFormat::Iso => format!("{year}-{month}-{day}"),
    };

    match calculate_journal_date_property(&page_id.name, &journal_title_metadata.today) {
        DateType::Today => {
            title = format!("{} (today)", title);
        }
        DateType::Yesterday => {
            title = format!("{} (yesterday)", title);
        }
        DateType::Tomorrow => {
            title = format!("{} (tomorrow)", title);
        }
        DateType::Other => {}
    }

    match journal_title_metadata
        .journal_configurataion
        .show_weekday_in_title
    {
        crate::looksyk::data::config::runtime_graph_configuration::ShowWeekdayInTitle::AsPrefix => {
            let weekday = calculate_weekday(year, month, day);
            title = format!("{} {}", weekday, title);
        }
        crate::looksyk::data::config::runtime_graph_configuration::ShowWeekdayInTitle::AsSuffix => {
            let weekday = calculate_weekday(year, month, day);
            title = format!("{} {}", title, weekday);
        }
        crate::looksyk::data::config::runtime_graph_configuration::ShowWeekdayInTitle::None => {}
    }

    let title_segments = vec![PageTitleSegment {
        title: title.clone(),
        link: journal_path(&page_id.name),
    }];
    PageTitle {
        title,
        title_segments,
    }
}

fn calculate_weekday(year: &str, month: &str, day: &str) -> String {
    let year: i32 = year.parse().unwrap_or(1970);
    let month: u32 = month.parse().unwrap_or(1);
    let day: u32 = day.parse().unwrap_or(1);
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        date.format("%A").to_string()
    } else {
        "Invalid date".to_string()
    }
}

pub struct JournalTitleCalculatorMetadata<'a> {
    pub journal_configurataion: &'a JournalConfigration,
    pub today: TodayContainer,
}

#[cfg(test)]
pub mod builder {
    use crate::io::date::builder::today;
    use crate::looksyk::data::config::runtime_graph_configuration::{
        JournalConfigration, JournalTitleFormat, ShowWeekdayInTitle,
    };

    pub fn world_journal_title_calculator_metadata(
    ) -> super::JournalTitleCalculatorMetadata<'static> {
        super::JournalTitleCalculatorMetadata {
            journal_configurataion: &JournalConfigration {
                journal_title_format: JournalTitleFormat::World,
                show_weekday_in_title: ShowWeekdayInTitle::None,
            },
            today: today(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::date::builder::{other_date, today, tomorrow, yesterday};
    use crate::looksyk::builder::test_builder::{journal_page_id, user_page_id};
    use crate::looksyk::data::config::runtime_graph_configuration::{
        JournalConfigration, JournalTitleFormat, ShowWeekdayInTitle,
    };
    use crate::looksyk::model::{PageId, PageType, SimplePageName};
    use crate::looksyk::renderer::title::builder::world_journal_title_calculator_metadata;
    use crate::looksyk::renderer::title::{calculate_page_title, JournalTitleCalculatorMetadata};

    #[test]
    fn test_calculate_page_title_world_format() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: SimplePageName {
                name: "2024_06_15".to_string(),
            },
        };
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::World,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: other_date(),
            },
        );
        assert_eq!(title.title, "15.06.2024");
    }
    #[test]
    fn test_calculate_page_title_american_format() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: SimplePageName {
                name: "2024_06_15".to_string(),
            },
        };
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::American,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: other_date(),
            },
        );
        assert_eq!(title.title, "06/15/2024");
    }

    #[test]
    fn test_calculate_page_title_iso_format() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: SimplePageName {
                name: "2024_06_15".to_string(),
            },
        };
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::Iso,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: other_date(),
            },
        );
        assert_eq!(title.title, "2024-06-15");
    }

    #[test]
    fn test_calculate_page_title_american_format_with_weekday_prefix() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: SimplePageName {
                name: "2024_06_15".to_string(),
            },
        };
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::American,
            show_weekday_in_title: ShowWeekdayInTitle::AsPrefix,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: other_date(),
            },
        );
        assert_eq!(title.title, "Saturday 06/15/2024");
    }

    #[test]
    fn test_calculate_page_title_world_format_with_weekday_suffix() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: SimplePageName {
                name: "2024_06_15".to_string(),
            },
        };
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::World,
            show_weekday_in_title: ShowWeekdayInTitle::AsSuffix,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: other_date(),
            },
        );
        assert_eq!(title.title, "15.06.2024 Saturday");
    }

    #[test]
    fn test_calculate_user_page_title_without_hierarchy() {
        let title = calculate_page_title(
            &user_page_id("MyPage"),
            &world_journal_title_calculator_metadata(),
        );
        assert_eq!(title.title, "MyPage");
        assert_eq!(title.title_segments.len(), 1);
        assert_eq!(title.title_segments[0].title, "MyPage");
        assert_eq!(title.title_segments[0].link, "page/MyPage");
    }

    #[test]
    fn test_calculate_user_page_title_with_hierarchy() {
        let title = calculate_page_title(
            &user_page_id("Folder/Subfolder/MyPage"),
            &world_journal_title_calculator_metadata(),
        );
        assert_eq!(title.title, "Folder/Subfolder/MyPage");
        assert_eq!(title.title_segments.len(), 3);
        assert_eq!(title.title_segments[0].title, "Folder");
        assert_eq!(title.title_segments[0].link, "page/Folder");
        assert_eq!(title.title_segments[1].title, "Subfolder");
        assert_eq!(title.title_segments[1].link, "page/Folder%2FSubfolder");
        assert_eq!(title.title_segments[2].title, "MyPage");
        assert_eq!(
            title.title_segments[2].link,
            "page/Folder%2FSubfolder%2FMyPage"
        );
    }

    #[test]
    fn test_calculate_user_page_title_with_hierarchy_should_trim_whitespace() {
        let title = calculate_page_title(
            &user_page_id("Folder / Subfolder / MyPage"),
            &world_journal_title_calculator_metadata(),
        );
        assert_eq!(title.title, "Folder / Subfolder / MyPage");
        assert_eq!(title.title_segments.len(), 3);
        assert_eq!(title.title_segments[0].title, "Folder");
        assert_eq!(title.title_segments[0].link, "page/Folder");
        assert_eq!(title.title_segments[1].title, "Subfolder");
        assert_eq!(
            title.title_segments[1].link,
            "page/Folder%20%2F%20Subfolder"
        );
        assert_eq!(title.title_segments[2].title, "MyPage");
        assert_eq!(
            title.title_segments[2].link,
            "page/Folder%20%2F%20Subfolder%20%2F%20MyPage"
        );
    }

    #[test]
    fn test_calculate_title_with_today_journal_entry() {
        let today = today();
        let page_id = journal_page_id(&format!(
            "{}_{:02}_{:02}",
            today.year(),
            today.month(),
            today.day()
        ));
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::Iso,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today,
            },
        );
        assert!(title.title.ends_with("(today)"));
        assert!(!title.title.ends_with("(tomorrow)"));
        assert!(!title.title.ends_with("(yesterday)"));
    }

    #[test]
    fn test_calculate_title_with_yesterday_journal_entry() {
        let yesterday = yesterday();
        let page_id = journal_page_id(&format!(
            "{}_{:02}_{:02}",
            yesterday.year(),
            yesterday.month(),
            yesterday.day()
        ));
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::Iso,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: today(),
            },
        );
        assert!(!title.title.ends_with("(today)"));
        assert!(!title.title.ends_with("(tomorrow)"));
        assert!(title.title.ends_with("(yesterday)"));
    }

    #[test]
    fn test_calculate_title_with_tomorrow_journal_entry() {
        let tomorrow = tomorrow();
        let page_id = journal_page_id(&format!(
            "{}_{:02}_{:02}",
            tomorrow.year(),
            tomorrow.month(),
            tomorrow.day()
        ));
        let design = JournalConfigration {
            journal_title_format: JournalTitleFormat::Iso,
            show_weekday_in_title: ShowWeekdayInTitle::None,
        };
        let title = calculate_page_title(
            &page_id,
            &JournalTitleCalculatorMetadata {
                journal_configurataion: &design,
                today: today(),
            },
        );
        assert!(!title.title.ends_with("(today)"));
        assert!(title.title.ends_with("(tomorrow)"));
        assert!(!title.title.ends_with("(yesterday)"));
    }
}
