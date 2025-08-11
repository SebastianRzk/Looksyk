use crate::http::state;
use crate::state::application_state::{AppState, GraphRootLocation};
use crate::sync::git::config::{GitConfig, GitSyncReadynessTrait};
use crate::sync::git::git_commands::RemoteUpdateResult::Error;
use crate::sync::git::git_commands::{
    check_if_remote_has_outgoing_updates, check_local_changes, check_remote_has_incoming_updates,
    git_commit, git_push, pull_updates_from_remote, RemoteUpdateResult,
};
use actix_web::web::Data;

pub fn create_checkpoint(
    git_config: &GitConfig,
    app_state: Option<Data<AppState>>,
    graph_root_location: &GraphRootLocation,
) -> GitActionResult {
    if !git_config.git_sync_readyness.is_ready() {
        return GitActionResult::error("Git configuration is not ready".to_string());
    }

    let create_checkpoint_result = git_commit(graph_root_location);
    if let Err(e) = &create_checkpoint_result {
        return GitActionResult::error(format!("Failed to create checkpoint: {e}"));
    }

    let update_result = check_remote_has_incoming_updates(graph_root_location);
    if let Error(e) = update_result {
        return GitActionResult::error(format!("Failed to check for updates on remote: {e}"));
    } else if update_result == RemoteUpdateResult::UpdatePending {
        let pull_result = pull_updates_from_remote(git_config, graph_root_location);
        if pull_result.is_err() {
            return GitActionResult::error(format!(
                "Failed to pull updates from remote: {}",
                pull_result.unwrap_err()
            ));
        } else {
            if let Some(app_state) = app_state {
                println!("Pulled updates from remote before pushing local changes.");
                println!("Updating internal state after pulling updates.");
                state::endpoints::refresh_internal_state(app_state);
                println!("Internal state updated successfully.");
            } else {
                println!("No application state provided, skipping internal state update.");
            }
        }
    }

    let git_push_result = git_push(graph_root_location);
    if let Err(e) = &git_push_result {
        return GitActionResult::error(format!("Failed to push: {e}"));
    }
    GitActionResult::success()
}

pub fn push_existing_commits(
    git_config: &GitConfig,
    app_state: Option<Data<AppState>>,
    graph_root_location: &GraphRootLocation,
) -> GitActionResult {
    if !git_config.git_sync_readyness.is_ready() {
        return GitActionResult::error("Git configuration is not ready".to_string());
    }

    let update_result = check_remote_has_incoming_updates(graph_root_location);

    if let Error(e) = update_result {
        return GitActionResult::error(format!("Failed to check for updates on remote: {e}"));
    } else if update_result == RemoteUpdateResult::UpdatePending {
        let pull_result = pull_updates_from_remote(git_config, graph_root_location);
        if pull_result.is_err() {
            return GitActionResult::error(format!(
                "Failed to pull updates from remote: {}",
                pull_result.unwrap_err()
            ));
        }
    }

    let create_checkpoint_result = git_push(graph_root_location);
    if let Err(e) = create_checkpoint_result {
        return GitActionResult::error(format!("Failed to push: {e}"));
    }
    if update_result == RemoteUpdateResult::UpdatePending {
        if let Some(app_state) = app_state {
            println!("Updating internal state.");
            state::endpoints::refresh_internal_state(app_state);
        } else {
            println!("No application state provided, skipping internal state update.");
        }
    }
    GitActionResult::success()
}

pub struct GitActionResult {
    pub success: bool,
    pub message: Option<String>,
}

impl GitActionResult {
    pub fn error(message: String) -> Self {
        GitActionResult {
            success: false,
            message: Some(message),
        }
    }

    pub fn success() -> Self {
        GitActionResult {
            message: None,
            success: true,
        }
    }
}

pub fn pull_updates(
    git_config: &GitConfig,
    app_state: Data<AppState>,
    graph_root_location: &GraphRootLocation,
) -> GitActionResult {
    let (update_result, pull_result) = match try_updating(git_config, graph_root_location) {
        Ok(value) => value,
        Err(value) => return value,
    };

    if let Err(e) = &pull_result {
        return GitActionResult::error(format!("Failed to pull updates from remote: {e}"));
    }

    if update_result == RemoteUpdateResult::UpdatePending {
        state::endpoints::refresh_internal_state(app_state);
    }

    GitActionResult::success()
}

pub fn try_updating(
    git_config: &GitConfig,
    graph_root_location: &GraphRootLocation,
) -> Result<(RemoteUpdateResult, Result<(), String>), GitActionResult> {
    if git_config.git_sync_readyness.not_ready() {
        return Err(GitActionResult::error(
            "Git configuration is not ready".to_string(),
        ));
    }

    let update_result = check_remote_has_incoming_updates(graph_root_location);

    if let Error(e) = update_result {
        return Err(GitActionResult::error(format!(
            "Failed to check for updates on remote: {e}"
        )));
    }

    let pull_result = pull_updates_from_remote(git_config, graph_root_location);
    Ok((update_result, pull_result))
}

pub fn calc_git_status(config: &GitConfig, graph_root_location: &GraphRootLocation) -> GitStatus {
    if config.git_sync_readyness.not_ready() {
        return GitStatus {
            enabled: config.enabled,
            has_changes: false,
            is_ready: false,
            has_error: false,
            has_incoming_updates: false,
            has_outgoing_updates: false,
        };
    }
    let has_updates_downstream = check_remote_has_incoming_updates(graph_root_location);
    let has_pending_updates_upstream = check_if_remote_has_outgoing_updates(graph_root_location);
    let has_changes = check_local_changes(graph_root_location);

    GitStatus {
        enabled: config.enabled,
        is_ready: true,
        has_error: has_updates_downstream.is_error()
            || has_pending_updates_upstream.is_error()
            || has_changes.is_err(),
        has_changes: has_changes.map_err(|_| true).unwrap(),
        has_incoming_updates: has_updates_downstream.is_update_pending(),
        has_outgoing_updates: has_pending_updates_upstream.is_update_pending(),
    }
}

pub struct GitStatus {
    pub enabled: bool,
    pub is_ready: bool,
    pub has_error: bool,
    pub has_changes: bool,
    pub has_incoming_updates: bool,
    pub has_outgoing_updates: bool,
}
