use crate::io::http::page::search::dtos::{
    SearchFindingDto, SearchReferenceDto, SearchResultDto, SearchTermDto,
};
use crate::looksyk::search::{SearchFinding, SearchResult, SearchTerm};
use crate::state::block::BlockReference;

pub fn search_result_to_dto(search_result: SearchResult) -> SearchResultDto {
    SearchResultDto {
        journal: search_result
            .journal
            .iter()
            .map(search_finding_to_dto)
            .collect(),
        page: search_result
            .page
            .iter()
            .map(search_finding_to_dto)
            .collect(),
    }
}

fn search_finding_to_dto(search_finding: &SearchFinding) -> SearchFindingDto {
    SearchFindingDto {
        reference: to_search_reference(&search_finding.reference),
        text_line: search_finding.text_line.clone(),
    }
}

fn to_search_reference(markdown_reference: &BlockReference) -> SearchReferenceDto {
    SearchReferenceDto {
        file_name: markdown_reference.page_id.name.name.clone(),
        block_number: markdown_reference.block_number,
    }
}

pub fn to_search_term(search_term_dto: SearchTermDto) -> SearchTerm {
    SearchTerm {
        as_string: search_term_dto.as_string,
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::page::search::mapper::{search_finding_to_dto, search_result_to_dto};
    use crate::looksyk::builder::test_builder::{journal_page_id, user_page_id};
    use crate::looksyk::search::{SearchFinding, SearchResult};
    use crate::state::block::BlockReference;

    #[test]
    fn test_search_finding_to_dto() {
        let search_finding = SearchFinding {
            reference: BlockReference {
                page_id: journal_page_id("page_id"),
                block_number: 0,
            },
            text_line: "text_line".to_string(),
        };

        let dto = search_finding_to_dto(&search_finding);

        assert_eq!(dto.text_line, "text_line");
        assert_eq!(dto.reference.file_name, "page_id");
    }

    #[test]
    fn test_search_result_to_dto() {
        let search_result = SearchResult {
            journal: vec![SearchFinding {
                reference: BlockReference {
                    page_id: journal_page_id("page_id"),
                    block_number: 0,
                },
                text_line: "text_line".to_string(),
            }],
            page: vec![SearchFinding {
                reference: BlockReference {
                    page_id: user_page_id("page_id"),
                    block_number: 0,
                },
                text_line: "text_line".to_string(),
            }],
        };

        let dto = search_result_to_dto(search_result);

        assert_eq!(dto.journal.len(), 1);
        assert_eq!(dto.page.len(), 1);
    }
}
