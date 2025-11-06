use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{get, web, Responder};

#[derive(serde::Serialize)]
pub struct BlockPropertiesDto {
    pub properties: Vec<String>,
}

#[get("/api/block_properties/")]
async fn get_block_properties(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let block_properties_guard = data.h_block_properties.lock().unwrap();
    let properties = block_properties_guard
        .get_all_keys()
        .into_iter()
        .map(|prop| prop.value)
        .collect();

    Ok(web::Json(BlockPropertiesDto { properties }))
}
