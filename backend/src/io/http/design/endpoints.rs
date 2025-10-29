use crate::io::fs::config::save_config_to_file;
use crate::looksyk::data::config::runtime_graph_configuration::Appearance;
use crate::state::application_state::AppState;
use crate::sync::io::sync_application_port::{document_change, GraphChange, GraphChangesState};
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse};
use std::str::FromStr;
use crate::io::http::design::dtos::{AppearanceDto, DesignConfigDto};

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

#[get("/api/appearance")]
pub async fn get_appearance(app_state: Data<AppState>) -> HttpResponse {
    let config = app_state.g_config.lock().unwrap();
    let appearance = config.design.appearance.clone();
    drop(config);
    HttpResponse::Ok().json(AppearanceDto {
        appearance: appearance.to_string(),
    })
}

#[post("/api/design-config")]
pub async fn set_design_config(
    app_state: Data<AppState>,
    design: web::Json<DesignConfigDto>,
    changes: web::Data<GraphChangesState>,
) -> HttpResponse {
    let mut config = app_state.g_config.lock().unwrap();
    config.design.primary_color = design.primary_color.clone();
    config.design.background_color = design.background_color.clone();
    config.design.foreground_color = design.foreground_color.clone();
    config.design.primary_shading = design.primary_shading.clone();
    config.design.appearance =
        Appearance::from_str(&design.appearance.appearance).unwrap_or(Appearance::Dark);
    save_config_to_file(&app_state.data_path, &config);
    drop(config);

    document_change(
        changes,
        GraphChange::configuration_changed("design configuration updated".to_string()),
    );
    HttpResponse::Ok().finish()
}
