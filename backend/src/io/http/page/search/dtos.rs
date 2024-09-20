use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SearchResultDto {
    pub journal: Vec<SearchFindingDto>,
    pub page: Vec<SearchFindingDto>,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFindingDto {
    pub reference: SearchReferenceDto,
    pub text_line: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTermDto {
    pub as_string: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchReferenceDto {
    pub file_name: String,
    pub block_number: usize,
}
