use crate::state::application_state::AppState;
use crate::sync::git::config::GitConfig;
use crate::sync::git::git_services::{
    calc_git_status, create_checkpoint, pull_updates, push_existing_commits, GitActionResult,
    GitStatus,
};
use actix_web::web::Data;
use actix_web::{get, web};
use actix_web::{post, Responder};
use serde::Serialize;

#[post("/api/sync/git/checkpoint")]
pub async fn post_create_checkpoint(
    data: Data<AppState>,
    git_config: Data<GitConfig>,
) -> actix_web::Result<impl Responder> {
    let location = data.data_path.clone();
    let create_checkpoint_result = create_checkpoint(&git_config, data, &location);
    Ok(web::Json(to_dto(create_checkpoint_result)))
}

fn to_dto(create_checkpoint_result: GitActionResult) -> GitActionResultDto {
    GitActionResultDto {
        success: create_checkpoint_result.success,
        message: create_checkpoint_result.message,
    }
}

#[derive(Debug, Serialize)]
struct GitActionResultDto {
    pub success: bool,
    pub message: Option<String>,
}

#[get("/api/sync/git/status")]
pub async fn get_current_git_status(
    git_config: Data<GitConfig>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let status = calc_git_status(&git_config, &data.data_path);
    Ok(web::Json::<GitStatusDto>(status.into()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GitStatusDto {
    pub enabled: bool,
    pub is_ready: bool,
    pub has_outgoing_updates: bool,
    pub has_incoming_updates: bool,
    pub has_changes: bool,
    pub has_errors: bool,
}

impl From<GitStatus> for GitStatusDto {
    fn from(status: GitStatus) -> Self {
        GitStatusDto {
            enabled: status.enabled,
            has_changes: status.has_changes,
            is_ready: status.is_ready,
            has_outgoing_updates: status.has_outgoing_updates,
            has_incoming_updates: status.has_incoming_updates,
            has_errors: status.has_error,
        }
    }
}

#[post("/api/sync/git/update")]
pub async fn update_current_data(
    git_config: Data<GitConfig>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let graph_root_location = data.data_path.clone();
    let updates = pull_updates(&git_config, data, &graph_root_location);
    Ok(web::Json(to_dto(updates)))
}

#[post("/api/sync/git/retry_upload")]
pub async fn post_retry_upload(
    git_config: Data<GitConfig>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let graph_root_location = data.data_path.clone();
    let updates = push_existing_commits(&git_config, data, &graph_root_location);
    Ok(web::Json(to_dto(updates)))
}
