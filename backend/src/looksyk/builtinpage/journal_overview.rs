use crate::looksyk::model::{ParsedBlock, ParsedMarkdownFile, SimplePageName};
use chrono::{Datelike, NaiveDate};

pub fn generate_journal_overview(journal_entries: Vec<SimplePageName>) -> ParsedMarkdownFile {
    let mut result = vec![];
    let mut sorted_journals = journal_entries
        .iter()
        .filter(|j| j.name.len() == 10)
        .map(|j| NaiveDate::parse_from_str(&j.name, "%Y_%m_%d").unwrap())
        .collect::<Vec<NaiveDate>>();
    sorted_journals.sort();
    if sorted_journals.is_empty() {
        return ParsedMarkdownFile {
            blocks: vec![ParsedBlock::artificial_text_block(
                "No journal entries found.",
            )],
        };
    }

    let min_date = sorted_journals[0];
    let max_date = sorted_journals[sorted_journals.len() - 1];
    let mut first = true;

    let mut as_text = "".to_string();
    let mut weekday = min_date.weekday().num_days_from_monday();
    for date in min_date.iter_days() {
        let first_of_month = date.day() == 1;

        if first_of_month || first {
            if !first {
                for _ in weekday..7 {
                    as_text.push_str(" | ");
                }
                result.push(ParsedBlock::artificial_text_block(&as_text));
                as_text = "".to_string();
            }

            if date.month() == 12 {
                as_text.push_str(format!("## {}\n\n", date.year()).as_str());
            }

            as_text.push_str(format!("### {}\n\n", date.format("%B %Y")).as_str());
            as_text.push_str("| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |\n| --- | --- | --- | --- | --- | --- | --- |\n|");
            for _ in 0..weekday {
                as_text.push_str(" | ");
            }
        }
        if weekday == 0 {
            as_text.push_str("\n|");
        }
        let date_url_str = date.format("%Y_%m_%d").to_string();
        let date_str = date.format("%d.").to_string();
        if !sorted_journals.contains(&date) {
            as_text.push_str(&format!(
                "<div class=\"cal-item\">[{date_str}](journal/{date_url_str})</div>|"
            ));
        } else {
            as_text.push_str(&format!("<div class=\"cal-item filled-cal-item\">[{date_str}](journal/{date_url_str})</div>|"));
        }
        weekday = (weekday + 1) % 7;

        if date >= max_date {
            break;
        }
        first = false;
    }

    for _ in weekday..7 {
        as_text.push_str(" | ");
    }

    result.push(ParsedBlock::artificial_text_block(&as_text));
    if max_date.month() != 1 {
        result.push(ParsedBlock::artificial_text_block(&format!(
            "## {}\n\n",
            max_date.format("%Y")
        )));
    }

    result.reverse();

    ParsedMarkdownFile { blocks: result }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::builder::test_builder::{
        extract_textblock_line_at, extract_very_first_textblock_line,
    };

    #[test]
    fn test_generate_journal_overview_with_entries() {
        let journal_entries = vec![page_name_str("2025_01_15"), page_name_str("2025_01_21")];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 1);
        assert_eq!(result.blocks[0].content.len(), 1);
        assert_eq!(extract_very_first_textblock_line(&result), "### January 2025

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  | <div class=\"cal-item filled-cal-item\">[15.](journal/2025_01_15)</div>|<div class=\"cal-item\">[16.](journal/2025_01_16)</div>|<div class=\"cal-item\">[17.](journal/2025_01_17)</div>|<div class=\"cal-item\">[18.](journal/2025_01_18)</div>|<div class=\"cal-item\">[19.](journal/2025_01_19)</div>|
|<div class=\"cal-item\">[20.](journal/2025_01_20)</div>|<div class=\"cal-item filled-cal-item\">[21.](journal/2025_01_21)</div>| |  |  |  |  | ");
    }

    #[test]
    fn test_generate_journal_overview_with_entries_and_monthbreak() {
        let journal_entries = vec![page_name_str("2025_01_15"), page_name_str("2025_03_21")];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 4);
        assert_eq!(result.blocks[0].content.len(), 1);
        assert_eq!(extract_very_first_textblock_line(&result), "## 2025\n\n");
        assert_eq!(result.blocks[1].content.len(), 1);
        assert_eq!(extract_textblock_line_at(&result, 1), "### March 2025

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  |  |  |  | <div class=\"cal-item\">[01.](journal/2025_03_01)</div>|<div class=\"cal-item\">[02.](journal/2025_03_02)</div>|
|<div class=\"cal-item\">[03.](journal/2025_03_03)</div>|<div class=\"cal-item\">[04.](journal/2025_03_04)</div>|<div class=\"cal-item\">[05.](journal/2025_03_05)</div>|<div class=\"cal-item\">[06.](journal/2025_03_06)</div>|<div class=\"cal-item\">[07.](journal/2025_03_07)</div>|<div class=\"cal-item\">[08.](journal/2025_03_08)</div>|<div class=\"cal-item\">[09.](journal/2025_03_09)</div>|
|<div class=\"cal-item\">[10.](journal/2025_03_10)</div>|<div class=\"cal-item\">[11.](journal/2025_03_11)</div>|<div class=\"cal-item\">[12.](journal/2025_03_12)</div>|<div class=\"cal-item\">[13.](journal/2025_03_13)</div>|<div class=\"cal-item\">[14.](journal/2025_03_14)</div>|<div class=\"cal-item\">[15.](journal/2025_03_15)</div>|<div class=\"cal-item\">[16.](journal/2025_03_16)</div>|
|<div class=\"cal-item\">[17.](journal/2025_03_17)</div>|<div class=\"cal-item\">[18.](journal/2025_03_18)</div>|<div class=\"cal-item\">[19.](journal/2025_03_19)</div>|<div class=\"cal-item\">[20.](journal/2025_03_20)</div>|<div class=\"cal-item filled-cal-item\">[21.](journal/2025_03_21)</div>| |  | ");

        assert_eq!(result.blocks[2].content.len(), 1);
        assert_eq!(extract_textblock_line_at(&result, 2), "### February 2025

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  |  |  |  | <div class=\"cal-item\">[01.](journal/2025_02_01)</div>|<div class=\"cal-item\">[02.](journal/2025_02_02)</div>|
|<div class=\"cal-item\">[03.](journal/2025_02_03)</div>|<div class=\"cal-item\">[04.](journal/2025_02_04)</div>|<div class=\"cal-item\">[05.](journal/2025_02_05)</div>|<div class=\"cal-item\">[06.](journal/2025_02_06)</div>|<div class=\"cal-item\">[07.](journal/2025_02_07)</div>|<div class=\"cal-item\">[08.](journal/2025_02_08)</div>|<div class=\"cal-item\">[09.](journal/2025_02_09)</div>|
|<div class=\"cal-item\">[10.](journal/2025_02_10)</div>|<div class=\"cal-item\">[11.](journal/2025_02_11)</div>|<div class=\"cal-item\">[12.](journal/2025_02_12)</div>|<div class=\"cal-item\">[13.](journal/2025_02_13)</div>|<div class=\"cal-item\">[14.](journal/2025_02_14)</div>|<div class=\"cal-item\">[15.](journal/2025_02_15)</div>|<div class=\"cal-item\">[16.](journal/2025_02_16)</div>|
|<div class=\"cal-item\">[17.](journal/2025_02_17)</div>|<div class=\"cal-item\">[18.](journal/2025_02_18)</div>|<div class=\"cal-item\">[19.](journal/2025_02_19)</div>|<div class=\"cal-item\">[20.](journal/2025_02_20)</div>|<div class=\"cal-item\">[21.](journal/2025_02_21)</div>|<div class=\"cal-item\">[22.](journal/2025_02_22)</div>|<div class=\"cal-item\">[23.](journal/2025_02_23)</div>|
|<div class=\"cal-item\">[24.](journal/2025_02_24)</div>|<div class=\"cal-item\">[25.](journal/2025_02_25)</div>|<div class=\"cal-item\">[26.](journal/2025_02_26)</div>|<div class=\"cal-item\">[27.](journal/2025_02_27)</div>|<div class=\"cal-item\">[28.](journal/2025_02_28)</div>| |  | ");
        assert_eq!(result.blocks[3].content.len(), 1);
        assert_eq!(extract_textblock_line_at(&result, 3), "### January 2025

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  | <div class=\"cal-item filled-cal-item\">[15.](journal/2025_01_15)</div>|<div class=\"cal-item\">[16.](journal/2025_01_16)</div>|<div class=\"cal-item\">[17.](journal/2025_01_17)</div>|<div class=\"cal-item\">[18.](journal/2025_01_18)</div>|<div class=\"cal-item\">[19.](journal/2025_01_19)</div>|
|<div class=\"cal-item\">[20.](journal/2025_01_20)</div>|<div class=\"cal-item\">[21.](journal/2025_01_21)</div>|<div class=\"cal-item\">[22.](journal/2025_01_22)</div>|<div class=\"cal-item\">[23.](journal/2025_01_23)</div>|<div class=\"cal-item\">[24.](journal/2025_01_24)</div>|<div class=\"cal-item\">[25.](journal/2025_01_25)</div>|<div class=\"cal-item\">[26.](journal/2025_01_26)</div>|
|<div class=\"cal-item\">[27.](journal/2025_01_27)</div>|<div class=\"cal-item\">[28.](journal/2025_01_28)</div>|<div class=\"cal-item\">[29.](journal/2025_01_29)</div>|<div class=\"cal-item\">[30.](journal/2025_01_30)</div>|<div class=\"cal-item\">[31.](journal/2025_01_31)</div>| |  | ");
    }

    #[test]
    fn test_generate_journal_overview_with_entries_and_yearbreak() {
        let journal_entries = vec![page_name_str("2024_12_15"), page_name_str("2025_01_21")];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 2);
        assert_eq!(result.blocks[0].content.len(), 1);
        assert_eq!(extract_very_first_textblock_line(&result), "### January 2025

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  | <div class=\"cal-item\">[01.](journal/2025_01_01)</div>|<div class=\"cal-item\">[02.](journal/2025_01_02)</div>|<div class=\"cal-item\">[03.](journal/2025_01_03)</div>|<div class=\"cal-item\">[04.](journal/2025_01_04)</div>|<div class=\"cal-item\">[05.](journal/2025_01_05)</div>|
|<div class=\"cal-item\">[06.](journal/2025_01_06)</div>|<div class=\"cal-item\">[07.](journal/2025_01_07)</div>|<div class=\"cal-item\">[08.](journal/2025_01_08)</div>|<div class=\"cal-item\">[09.](journal/2025_01_09)</div>|<div class=\"cal-item\">[10.](journal/2025_01_10)</div>|<div class=\"cal-item\">[11.](journal/2025_01_11)</div>|<div class=\"cal-item\">[12.](journal/2025_01_12)</div>|
|<div class=\"cal-item\">[13.](journal/2025_01_13)</div>|<div class=\"cal-item\">[14.](journal/2025_01_14)</div>|<div class=\"cal-item\">[15.](journal/2025_01_15)</div>|<div class=\"cal-item\">[16.](journal/2025_01_16)</div>|<div class=\"cal-item\">[17.](journal/2025_01_17)</div>|<div class=\"cal-item\">[18.](journal/2025_01_18)</div>|<div class=\"cal-item\">[19.](journal/2025_01_19)</div>|
|<div class=\"cal-item\">[20.](journal/2025_01_20)</div>|<div class=\"cal-item filled-cal-item\">[21.](journal/2025_01_21)</div>| |  |  |  |  | ");
        assert_eq!(result.blocks[1].content.len(), 1);
        assert_eq!(extract_textblock_line_at(&result, 1), "## 2024

### December 2024

| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  |  |  |  |  | <div class=\"cal-item filled-cal-item\">[15.](journal/2024_12_15)</div>|
|<div class=\"cal-item\">[16.](journal/2024_12_16)</div>|<div class=\"cal-item\">[17.](journal/2024_12_17)</div>|<div class=\"cal-item\">[18.](journal/2024_12_18)</div>|<div class=\"cal-item\">[19.](journal/2024_12_19)</div>|<div class=\"cal-item\">[20.](journal/2024_12_20)</div>|<div class=\"cal-item\">[21.](journal/2024_12_21)</div>|<div class=\"cal-item\">[22.](journal/2024_12_22)</div>|
|<div class=\"cal-item\">[23.](journal/2024_12_23)</div>|<div class=\"cal-item\">[24.](journal/2024_12_24)</div>|<div class=\"cal-item\">[25.](journal/2024_12_25)</div>|<div class=\"cal-item\">[26.](journal/2024_12_26)</div>|<div class=\"cal-item\">[27.](journal/2024_12_27)</div>|<div class=\"cal-item\">[28.](journal/2024_12_28)</div>|<div class=\"cal-item\">[29.](journal/2024_12_29)</div>|
|<div class=\"cal-item\">[30.](journal/2024_12_30)</div>|<div class=\"cal-item\">[31.](journal/2024_12_31)</div>| |  |  |  |  | ");
    }

    #[test]
    fn test_generate_journal_overview_empty() {
        let journal_entries = vec![];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 1);
        assert_eq!(
            result.blocks[0].content[0]
                .as_tokens
                .get(0)
                .unwrap()
                .payload,
            "No journal entries found."
        );
    }
}
