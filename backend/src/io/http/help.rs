use crate::io::date::today;
use crate::io::http::page::mapper::map_markdown_file_to_dto;
use crate::looksyk::builtinpage::help_page::help_page;
use crate::looksyk::model::PageTitle;
use crate::looksyk::renderer::title::JournalTitleCalculatorMetadata;
use crate::state::application_state::AppState;
use actix_web::web::{Data, Json};
use actix_web::{get, Responder};

#[get("/api/builtin-pages/help")]
async fn help(data: Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(Json(map_markdown_file_to_dto(
        help_page(&JournalTitleCalculatorMetadata {
            today: today(),
            journal_configurataion: &data.g_config.lock().unwrap().journal_configuration,
        }),
        false,
        PageTitle::internal_page_title("Help".to_string()),
    )))
}
