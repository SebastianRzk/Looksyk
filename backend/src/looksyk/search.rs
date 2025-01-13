use crate::looksyk::model::{MarkdownReference, PageType, ParsedMarkdownFile, SimplePageName};
use crate::state::journal::JournalPageIndex;
use crate::state::userpage::UserPageIndex;
use std::collections::HashMap;

pub struct SearchTerm {
    pub as_string: String,
}

pub struct SearchResult {
    pub journal: Vec<SearchFinding>,
    pub page: Vec<SearchFinding>,
}

pub struct SearchFinding {
    pub reference: MarkdownReference,
    pub text_line: String,
}

pub fn search(
    search_term: SearchTerm,
    journal_page_index: &JournalPageIndex,
    user_page_index: &UserPageIndex,
) -> SearchResult {
    SearchResult {
        page: search_in_index(&search_term, PageType::UserPage, &user_page_index.entries),
        journal: search_in_index(
            &search_term,
            PageType::JournalPage,
            &journal_page_index.entries,
        ),
    }
}

fn search_in_index(
    search_term: &SearchTerm,
    page_type: PageType,
    pages: &HashMap<SimplePageName, ParsedMarkdownFile>,
) -> Vec<SearchFinding> {
    let mut result = vec![];

    for (simple_page_name, parsed_markdown_file) in pages.iter() {
        let mut block_number = 0;
        for block in parsed_markdown_file.blocks.iter() {
            for block_content in &block.content {
                if block_content.as_text.contains(&search_term.as_string) {
                    result.push(SearchFinding {
                        reference: MarkdownReference {
                            page_id: simple_page_name.as_page_id(&page_type),
                            block_number,
                        },
                        text_line: block_content.as_text.clone(),
                    });
                }
            }
            block_number += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::looksyk::model::{
        BlockContent, PageType, ParsedBlock, ParsedMarkdownFile, SimplePageName,
    };
    use crate::looksyk::search::SearchTerm;
    use std::collections::HashMap;

    #[test]
    fn test_search_in_index() {
        let search_term = SearchTerm {
            as_string: "search".to_string(),
        };

        let mut pages = HashMap::new();
        let mut page = ParsedMarkdownFile { blocks: vec![] };
        page.blocks.push(ParsedBlock {
            content: vec![BlockContent {
                as_text: "asf search alkj".to_string(),
                as_tokens: vec![],
            }],
            indentation: 0,
        });
        pages.insert(
            SimplePageName {
                name: "page_name".to_string(),
            },
            page,
        );

        let result = super::search_in_index(&search_term, PageType::UserPage, &pages);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text_line, "asf search alkj");
    }
}
