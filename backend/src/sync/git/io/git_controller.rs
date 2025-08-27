use crate::io::http::state::endpoints::refresh_internal_state;
use crate::state::application_state::AppState;
use crate::sync::git::application_port::git_sync_application_port::CommitInitiator;
use crate::sync::git::config::{GitConfigData, GitConflictResolution};
use crate::sync::git::git_services::{
    calc_git_status, connect_to_empty_git_repository, create_checkpoint, pull_updates,
    push_existing_commits, setup_remote_graph, GitActionResult, GitConnect, GitStatus,
};
use crate::sync::git::io::git_config::{
    initialize_inner_git_config, save_git_config_to_disk, GitConfigOnDisk,
};
use crate::sync::io::sync_application_port::GraphChangesState;
use actix_web::web::Data;
use actix_web::{get, web};
use actix_web::{post, Responder};
use serde::{Deserialize, Serialize};

#[post("/api/sync/git/checkpoint")]
pub async fn post_create_checkpoint(
    data: Data<AppState>,
    git_config: Data<GitConfigData>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let location = data.data_path.clone();
    let mut graph_changes = graph_changes.changes.lock().unwrap();
    let create_checkpoint_result = create_checkpoint(
        &git_config.config.lock().unwrap(),
        Some(data),
        &location,
        CommitInitiator::UserCheckpoint,
        &graph_changes,
    );
    graph_changes.clear();
    Ok(web::Json(to_dto(create_checkpoint_result)))
}

#[post("/api/sync/git/shutdown-checkpoint")]
pub async fn post_create_shutdown_checkpoint(
    data: Data<AppState>,
    git_config: Data<GitConfigData>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let location = data.data_path.clone();
    let mut graph_changes = graph_changes.changes.lock().unwrap();
    let create_checkpoint_result = create_checkpoint(
        &git_config.config.lock().unwrap(),
        Some(data),
        &location,
        CommitInitiator::Shutdown,
        &graph_changes,
    );
    graph_changes.clear();
    Ok(web::Json(to_dto(create_checkpoint_result)))
}

fn to_dto(create_checkpoint_result: GitActionResult) -> GitActionResultDto {
    GitActionResultDto {
        success: create_checkpoint_result.success,
        message: create_checkpoint_result.message,
        changes_pulled_from_remote: create_checkpoint_result.changes_from_remote,
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GitActionResultDto {
    pub success: bool,
    pub message: Option<String>,
    pub changes_pulled_from_remote: bool,
}

#[get("/api/sync/git/status")]
pub async fn get_current_git_status(
    git_config: Data<GitConfigData>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let status = calc_git_status(&git_config.config.lock().unwrap(), &data.data_path);
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
    pub last_commit: String,
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
            last_commit: status.last_commit,
        }
    }
}

#[post("/api/sync/git/update")]
pub async fn update_current_data(
    git_config: Data<GitConfigData>,
    data: Data<AppState>,
    graph_changes: Data<GraphChangesState>,
) -> actix_web::Result<impl Responder> {
    let graph_root_location = data.data_path.clone();
    let mut graph_changes = graph_changes.changes.lock().unwrap();
    let updates = pull_updates(
        &git_config.config.lock().unwrap(),
        data,
        &graph_root_location,
        CommitInitiator::UserUpdate,
        &graph_changes,
    );
    if updates.commit_was_done {
        graph_changes.clear();
    }
    Ok(web::Json(to_dto(updates)))
}

#[post("/api/sync/git/retry_upload")]
pub async fn post_retry_upload(
    git_config: Data<GitConfigData>,
    data: Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let graph_root_location = data.data_path.clone();
    let updates = push_existing_commits(
        &git_config.config.lock().unwrap(),
        Some(data),
        &graph_root_location,
    );
    Ok(web::Json(to_dto(updates)))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRepoDto {
    pub url: String,
    pub git_conflict_resolution: String,
    pub halt_on_migration_without_internet: bool,
}

#[post("/api/sync/git/clone_existing_git")]
pub async fn post_clone_existing_graph(
    data: Data<AppState>,
    git_config: Data<GitConfigData>,
    git_repo: web::Json<GitRepoDto>,
) -> actix_web::Result<impl Responder> {
    let new_git_config = &GitConfigOnDisk {
        active: true,
        halt_on_migration_without_internet: git_repo.halt_on_migration_without_internet,
        git_conflict_resolution: GitConflictResolution::from(&git_repo.git_conflict_resolution)
            .to_string(),
    };

    let result = setup_remote_graph(&data.data_path, &git_repo.url);
    match result {
        GitConnect::ConnectedSuccessfully => {
            save_git_config_to_disk(&data.data_path, new_git_config);
            let new_git_config_data = initialize_inner_git_config(&new_git_config, &data.data_path);
            *git_config.config.lock().unwrap() = new_git_config_data;
            refresh_internal_state(data);
            Ok(web::Json(GitActionResultDto {
                success: true,
                message: Some("Git repository cloned successfully.".to_string()),
                changes_pulled_from_remote: true,
            }))
        }
        GitConnect::ConnectFailed(e) => Ok(web::Json(GitActionResultDto {
            success: false,
            message: Some(format!("Failed to clone git repository: {}", e)),
            changes_pulled_from_remote: true,
        })),
    }
}

#[post("/api/sync/git/connect")]
pub async fn post_connect_to_git(
    data: Data<AppState>,
    git_config: Data<GitConfigData>,
    git_repo: web::Json<GitRepoDto>,
) -> actix_web::Result<impl Responder> {
    let new_git_config = &GitConfigOnDisk {
        active: true,
        halt_on_migration_without_internet: git_repo.halt_on_migration_without_internet,
        git_conflict_resolution: GitConflictResolution::from(&git_repo.git_conflict_resolution)
            .to_string(),
    };

    let result = connect_to_empty_git_repository(&data.data_path, &git_repo.url);
    match result {
        GitConnect::ConnectedSuccessfully => {
            save_git_config_to_disk(&data.data_path, new_git_config);
            let new_git_config_data = initialize_inner_git_config(&new_git_config, &data.data_path);
            *git_config.config.lock().unwrap() = new_git_config_data;
            refresh_internal_state(data);
            Ok(web::Json(GitActionResultDto {
                success: true,
                message: Some("Connected to remote git repository successfully.".to_string()),
                changes_pulled_from_remote: false,
            }))
        }
        GitConnect::ConnectFailed(e) => Ok(web::Json(GitActionResultDto {
            success: false,
            message: Some(format!("Failed to connect to remote git repository: {}", e)),
            changes_pulled_from_remote: false,
        })),
    }
}
