use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePageDto {
    pub old_page_name: String,
    pub new_page_name: String,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePageResultDto {
    pub new_page_name: String,
}

#[derive(Serialize)]
pub struct PageDeletedDto {
}