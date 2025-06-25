use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};

use crate::state::application_state::AppState;

#[get("/api/design")]
pub async fn get_css_theme(app_state: Data<AppState>) -> HttpResponse {
    let config = app_state.g_config.lock().unwrap();
    let design = &config.design;
    let css_text = format!(
        "
:root {{
--primary-color: {};
--background-color: {};
--foreground-color: {};
--primary-shading: {};
}}
",
        design.primary_color,
        design.background_color,
        design.foreground_color,
        design.primary_shading
    );
    drop(config);
    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_CSS))
        .body(css_text)
}

#[derive(serde::Serialize)]
struct AppearanceDto {
    appearance: String,
}

#[get("/api/appearance")]
pub async fn get_appearance(app_state: Data<AppState>) -> HttpResponse {
    let config = app_state.g_config.lock().unwrap();
    let appearance = config.appearance.clone();
    drop(config);
    HttpResponse::Ok().json(AppearanceDto { appearance: appearance.to_string() })
}
