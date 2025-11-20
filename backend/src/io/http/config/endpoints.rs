use crate::io::fs::config::save_config_to_file;
use crate::io::http::config::dtos::JournalConfigurationDto;
use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{get, post, web, Responder};

#[get("/api/config/journal")]
async fn get_journal_title_format(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    let dto: JournalConfigurationDto =
        (&data.g_config.lock().unwrap().journal_configuration).into();
    Ok(web::Json(dto))
}

#[post("/api/config/journal")]
async fn set_journal_title_format(
    data: Data<AppState>,
    new_config: web::Json<JournalConfigurationDto>,
) -> actix_web::Result<impl Responder> {
    let mut config = data.g_config.lock().unwrap();

    config.journal_configuration = new_config.into_inner().into();
    save_config_to_file(&data.data_path, &config);

    let dto: JournalConfigurationDto = (&config.journal_configuration).into();
    Ok(web::Json(dto))
}
