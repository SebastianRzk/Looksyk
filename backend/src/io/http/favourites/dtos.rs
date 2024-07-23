use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FavListDto {
    pub list: Vec<String>
}
