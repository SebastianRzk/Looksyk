use actix_web::{get, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::web::Data;

use crate::state::state::AppState;

#[get("/api/design")]
pub async fn get_css_theme(app_state: Data<AppState>) -> HttpResponse {
    let config = app_state.g_config.lock().unwrap();
    let design = &config.design;
    let css_text = format!("
:root {{
--primary-color: {};
--background-color: {};
--foreground-color: {};
--primary-shading: {};
}}
", design.primary_color, design.background_color, design.foreground_color, design.primary_shading);
    drop(config);
    HttpResponse::Ok().insert_header(ContentType(mime::TEXT_CSS)).body(css_text)
}


