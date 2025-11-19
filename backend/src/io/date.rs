use crate::looksyk::model::SimplePageName;
use crate::looksyk::title::DateType;
#[cfg(test)]
use chrono::Datelike;
use chrono::{Local, NaiveDate};

pub fn today() -> TodayContainer {
    TodayContainer {
        today: Local::now().date_naive(),
    }
}

pub struct TodayContainer {
    today: NaiveDate,
}

pub fn calculate_journal_date_property(name: &SimplePageName, today: TodayContainer) -> DateType {
    let splitted_date: Vec<&str> = name.name.split("_").collect();
    let year = splitted_date[0];
    let month = splitted_date[1];
    let day = splitted_date[2];
    let date = chrono::NaiveDate::from_ymd_opt(
        year.parse::<i32>().unwrap(),
        month.parse::<u32>().unwrap(),
        day.parse::<u32>().unwrap(),
    )
    .expect("Invalid date format");

    if date == today.today {
        DateType::Today
    } else if date == today.today + chrono::Duration::days(1) {
        DateType::Tomorrow
    } else if date == today.today - chrono::Duration::days(1) {
        DateType::Yesterday
    } else {
        DateType::Other
    }
}

#[cfg(test)]
pub mod builder {
    use crate::io::date::TodayContainer;
    use chrono::NaiveDate;

    pub fn today() -> TodayContainer {
        TodayContainer {
            today: NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        }
    }

    pub fn yesterday() -> TodayContainer {
        TodayContainer {
            today: NaiveDate::from_ymd_opt(2024, 6, 14).unwrap(),
        }
    }

    pub fn tomorrow() -> TodayContainer {
        TodayContainer {
            today: NaiveDate::from_ymd_opt(2024, 6, 16).unwrap(),
        }
    }

    pub fn other_date() -> TodayContainer {
        TodayContainer {
            today: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
        }
    }
}

#[cfg(test)]
impl TodayContainer {
    pub fn year(&self) -> i32 {
        self.today.year()
    }
    pub fn month(&self) -> u32 {
        self.today.month()
    }
    pub fn day(&self) -> u32 {
        self.today.day()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::date::builder::today;
    use crate::io::date::calculate_journal_date_property;
    use crate::looksyk::model::SimplePageName;
    use crate::looksyk::title::DateType;

    #[test]
    fn test_get_journal_date_property_for_today() {
        let today_page = SimplePageName {
            name: "2024_06_15".to_string(),
        };
        assert_eq!(
            calculate_journal_date_property(&today_page, today()),
            DateType::Today
        );
    }

    #[test]
    fn test_get_journal_date_property_for_tomorrow() {
        let tomorrow_page = SimplePageName {
            name: "2024_06_16".to_string(),
        };

        assert_eq!(
            calculate_journal_date_property(&tomorrow_page, today()),
            DateType::Tomorrow
        );
    }

    #[test]
    fn test_get_journal_date_property_for_yesterday() {
        let yesterday_page = SimplePageName {
            name: "2024_06_14".to_string(),
        };

        assert_eq!(
            calculate_journal_date_property(&yesterday_page, today()),
            DateType::Yesterday
        );
    }

    #[test]
    fn test_get_journal_date_property_for_other() {
        let other_page = SimplePageName {
            name: "2024_06_10".to_string(),
        };

        assert_eq!(
            calculate_journal_date_property(&other_page, today()),
            DateType::Other
        );
    }
}
