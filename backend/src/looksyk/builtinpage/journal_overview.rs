use crate::looksyk::model::{
    BlockContent, BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, SimplePageName,
};
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
            blocks: vec![ParsedBlock {
                indentation: 0,
                content: vec![BlockContent {
                    as_text: "No journal entries found.".to_string(),
                    as_tokens: vec![BlockToken {
                        block_token_type: BlockTokenType::Text,
                        payload: "No journal entries found.".to_string(),
                    }],
                }],
            }],
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
                result.push(ParsedBlock {
                    indentation: 0,
                    content: vec![BlockContent {
                        as_text: as_text.clone(),
                        as_tokens: vec![BlockToken {
                            block_token_type: BlockTokenType::Text,
                            payload: as_text,
                        }],
                    }],
                });
                as_text = "".to_string();
            }

            if date.month() == 12 || first {
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
        let date_str = date.format("%Y_%m_%d").to_string();
        if !sorted_journals.contains(&date) {
            as_text.push_str(&format!("[No Entry](/journal/{date_str})|"));
        } else {
            as_text.push_str(&format!("[{date_str}](/journal/{date_str})|"));
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

    result.push(ParsedBlock {
        indentation: 0,
        content: vec![BlockContent {
            as_text: as_text.clone(),
            as_tokens: vec![BlockToken {
                block_token_type: BlockTokenType::Text,
                payload: as_text,
            }],
        }],
    });
    if max_date.month() != 1 {
        result.push(ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: format!("## {}\n\n", max_date.format("%Y")),
                as_tokens: vec![BlockToken {
                    block_token_type: BlockTokenType::Text,
                    payload: format!("## {}\n\n", max_date.format("%Y")),
                }],
            }],
        });
    }

    result.reverse();

    ParsedMarkdownFile { blocks: result }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::page_name_str;

    #[test]
    fn test_generate_journal_overview() {
        let journal_entries = vec![page_name_str("2025_01_15"), page_name_str("2025_01_21")];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 1);
        assert_eq!(result.blocks[0].content.len(), 1);
        assert_eq!(result.blocks[0].content[0].as_text, "## 2025\n\n### January 2025\n\n| Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday |
| --- | --- | --- | --- | --- | --- | --- |
| |  | [2025_01_15](/journal/2025_01_15)|[No Entry](/journal/2025_01_16)|[No Entry](/journal/2025_01_17)|[No Entry](/journal/2025_01_18)|[No Entry](/journal/2025_01_19)|
|[No Entry](/journal/2025_01_20)|[2025_01_21](/journal/2025_01_21)| |  |  |  |  | ");
    }

    #[test]
    fn test_generate_journal_overview_empty() {
        let journal_entries = vec![];

        let result = super::generate_journal_overview(journal_entries);

        assert_eq!(result.blocks.len(), 1);
        assert_eq!(
            result.blocks[0].content[0].as_text,
            "No journal entries found."
        );
    }
}
