use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FavListDto {
    pub list: Vec<FavDto>,
}

#[derive(Serialize, Deserialize)]
pub struct FavDto {
    pub name: String,
    pub url: String
}


#[derive( Deserialize)]
pub struct FavUrlDto {
    pub url: String
}