use crate::http::state;
use crate::state::application_state::{AppState, GraphRootLocation};
use crate::sync::git::application_port::git_sync_application_port::CommitInitiator;
use crate::sync::git::config::{GitConfig, GitSyncReadynessTrait};
use crate::sync::git::git_commands::RemoteUpdateResult::Error;
use crate::sync::git::git_commands::{
    check_if_remote_has_outgoing_updates, check_local_changes, check_remote_has_incoming_updates,
    git_commit, git_push, pull_updates_from_remote, RemoteUpdateResult,
};
use crate::sync::git::git_services::UpdateResult::NothingToDo;
use crate::sync::io::sync_application_port::GraphChange;
use actix_web::web::Data;
use std::collections::HashSet;

pub fn create_checkpoint(
    git_config: &GitConfig,
    app_state: Option<Data<AppState>>,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &HashSet<GraphChange>,
) -> GitActionResult {
    if !git_config.git_sync_readyness.is_ready() {
        return GitActionResult::error("Git configuration is not ready".to_string());
    }

    let create_checkpoint_result = git_commit(
        graph_root_location,
        calculate_git_commit_message(initiator, graph_changes),
    );
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
        } else if let Some(app_state) = app_state {
            println!("Pulled updates from remote before pushing local changes.");
            println!("Updating internal state after pulling updates.");
            state::endpoints::refresh_internal_state(app_state);
            println!("Internal state updated successfully.");
        } else {
            println!("No application state provided, skipping internal state update.");
        }
    }

    let git_push_result = git_push(graph_root_location);
    if let Err(e) = &git_push_result {
        return GitActionResult::error(format!("Failed to push: {e}"));
    }
    GitActionResult::success(true)
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
    GitActionResult::success(false)
}

pub struct GitActionResult {
    pub success: bool,
    pub commit_was_done: bool,
    pub message: Option<String>,
}

impl GitActionResult {
    pub fn error(message: String) -> Self {
        GitActionResult {
            success: false,
            commit_was_done: false,
            message: Some(message),
        }
    }

    pub fn success(commit_was_done: bool) -> Self {
        GitActionResult {
            message: None,
            commit_was_done,
            success: true,
        }
    }
}

pub fn pull_updates(
    git_config: &GitConfig,
    app_state: Data<AppState>,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &HashSet<GraphChange>,
) -> GitActionResult {
    let try_updating_result =
        try_updating(git_config, graph_root_location, initiator, graph_changes);
    if let UpdateResult::Error(e) = &try_updating_result {
        return GitActionResult::error(format!("Failed to pull updates from remote: {e}"));
    }

    if try_updating_result == UpdateResult::HasChangedSomething {
        state::endpoints::refresh_internal_state(app_state);
    }

    GitActionResult::success(try_updating_result == UpdateResult::HasChangedSomething)
}

pub fn try_updating(
    git_config: &GitConfig,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &HashSet<GraphChange>,
) -> UpdateResult {
    if git_config.git_sync_readyness.not_ready() {
        return NothingToDo;
    }

    let fetch_result = check_remote_has_incoming_updates(graph_root_location);

    if let Error(e) = fetch_result {
        return UpdateResult::Error(format!("Failed to check for updates on remote: {e}"));
    }

    if fetch_result == RemoteUpdateResult::UpdatePending {
        let has_local_changes_result = check_local_changes(graph_root_location);
        if let Err(e) = &has_local_changes_result {
            return UpdateResult::Error(format!("Failed to check local changes: {e}"));
        }

        let local_commit_result = if has_local_changes_result.unwrap() {
            let result = git_commit(
                graph_root_location,
                calculate_git_commit_message(initiator, graph_changes),
            );
            if let Err(e) = &result {
                return UpdateResult::Error(format!("Failed to commit local changes: {e}"));
            }
            true
        } else {
            false
        };

        let pull_result = pull_updates_from_remote(git_config, graph_root_location);
        if let Err(e) = &pull_result {
            return UpdateResult::Error(format!("Failed to pull updates from remote: {e}"));
        }

        if local_commit_result {
            let push_result = git_push(graph_root_location);
            if let Err(e) = &push_result {
                return UpdateResult::Error(format!("Failed to push local changes: {e}"));
            }
        }
        UpdateResult::HasChangedSomething
    } else {
        UpdateResult::NothingToDo
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateResult {
    HasChangedSomething,
    Error(String),
    NothingToDo,
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

impl CommitInitiator {
    pub fn description(&self) -> &str {
        match self {
            CommitInitiator::Startup => "Application Startup",
            CommitInitiator::Shutdown => "Application Shutdown",
            CommitInitiator::UserCheckpoint => "User Checkpoint",
            CommitInitiator::UserUpdate => "User Update",
            CommitInitiator::Migration => "Application Version Migration",
        }
    }
}

fn calculate_git_commit_message(
    commit_entity: CommitInitiator,
    graph_changes: &HashSet<GraphChange>,
) -> String {
    let headline = calculate_headline(commit_entity, graph_changes);
    let body = calculate_body(graph_changes);

    format!("{headline}\n\n{body}")
}

fn calculate_headline(
    commit_initiator: CommitInitiator,
    graph_changes: &HashSet<GraphChange>,
) -> String {
    let content = graph_changes
        .iter()
        .map(|change| change.change_type.description())
        .collect::<Vec<String>>()
        .join(", ");

    format!("{}: {}", commit_initiator.description(), content)
}

fn calculate_body(graph_changes: &HashSet<GraphChange>) -> String {
    let mut sorted_changes: Vec<&GraphChange> = graph_changes.iter().collect();
    sorted_changes.sort_by(|l, r| l.target.cmp(&r.target));

    let changes = sorted_changes
        .iter()
        .map(|change| {
            format!(
                "  * {}: {}",
                change.change_type.description(),
                change.target
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!("Changes:\n{changes}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::io::sync_application_port::GraphChangeType;

    #[test]
    fn test_calculate_git_commit_message() {
        let changes = HashSet::from([
            GraphChange {
                change_type: GraphChangeType::PageChanged,
                target: "Page1".to_string(),
            },
            GraphChange {
                change_type: GraphChangeType::PageRenamed,
                target: "Page2".to_string(),
            },
        ]);

        let message = calculate_git_commit_message(CommitInitiator::UserCheckpoint, &changes);

        assert_eq!(message, ("User Checkpoint: Page changed, Page renamed\n\nChanges:\n  * Page changed: Page1\n  * Page renamed: Page2"));
    }
}
