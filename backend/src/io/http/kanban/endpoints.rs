use crate::io::http::page::dtos::UpdateBlockContentDto;
use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{get, web, Responder};

#[get("/api/kanban/")]
async fn get_kanban(
    body: web::Json<UpdateBlockContentDto>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    
    


    Ok(web::Json("kanban data"))
}
