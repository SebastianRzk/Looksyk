use crate::http::state;
use crate::state::application_state::{AppState, GraphRootLocation};
use crate::sync::git::application_port::git_sync_application_port::CommitInitiator;
use crate::sync::git::config::{GitConfig, GitSyncReadynessTrait};
use crate::sync::git::git_commands::RemoteUpdateResult::Error;
use crate::sync::git::git_commands::{
    check_if_remote_has_outgoing_updates, check_local_changes, check_remote_has_incoming_updates,
    get_last_commit_timestamp, git_add_remote, git_clone, git_commit, git_config_default_merge,
    git_config_default_no_edit, git_config_push_default, git_config_user_email,
    git_config_user_name, git_init, git_push, pull_updates_from_remote, RemoteUpdateResult,
};
use crate::sync::git::git_services::UpdateResult::NothingToDo;
use crate::sync::io::sync_application_port::{GraphChange, GraphChanges};
use actix_web::web::Data;
use std::collections::HashSet;

pub fn create_checkpoint(
    git_config: &GitConfig,
    app_state: Option<Data<AppState>>,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &GraphChanges,
) -> GitActionResult {
    let mut changes_from_remote = false;
    if !git_config.git_sync_readyness.is_ready() {
        return GitActionResult::error(
            "Git configuration is not ready".to_string(),
            changes_from_remote,
        );
    }

    let create_checkpoint_result = git_commit(
        graph_root_location,
        calculate_git_commit_message(initiator, graph_changes),
    );
    if let Err(e) = &create_checkpoint_result {
        return GitActionResult::error(
            format!("Failed to create checkpoint: {e}"),
            changes_from_remote,
        );
    }

    let update_result = check_remote_has_incoming_updates(graph_root_location);
    if let Error(e) = update_result {
        return GitActionResult::error(
            format!("Failed to check for updates on remote: {e}"),
            changes_from_remote,
        );
    } else if update_result == RemoteUpdateResult::UpdatePending {
        let pull_result = pull_updates_from_remote(git_config, graph_root_location);
        if pull_result.is_err() {
            return GitActionResult::error(
                format!(
                    "Failed to pull updates from remote: {}",
                    pull_result.unwrap_err()
                ),
                changes_from_remote,
            );
        } else if let Some(app_state) = app_state {
            println!("Pulled updates from remote before pushing local changes.");
            println!("Updating internal state after pulling updates.");
            state::endpoints::refresh_internal_state(app_state);
            changes_from_remote = true;
            println!("Internal state updated successfully.");
        } else {
            println!("No application state provided, skipping internal state update.");
        }
    }

    let git_push_result = git_push(graph_root_location);
    if let Err(e) = &git_push_result {
        return GitActionResult::error(format!("Failed to push: {e}"), changes_from_remote);
    }
    GitActionResult::success(true, changes_from_remote)
}

pub fn push_existing_commits(
    git_config: &GitConfig,
    app_state: Option<Data<AppState>>,
    graph_root_location: &GraphRootLocation,
) -> GitActionResult {
    if !git_config.git_sync_readyness.is_ready() {
        return GitActionResult::error("Git configuration is not ready".to_string(), false);
    }

    let update_result = check_remote_has_incoming_updates(graph_root_location);

    let changes_pulled_from_remote = update_result == RemoteUpdateResult::UpdatePending;
    if let Error(e) = update_result {
        return GitActionResult::error(
            format!("Failed to check for updates on remote: {e}"),
            changes_pulled_from_remote,
        );
    } else if changes_pulled_from_remote {
        let pull_result = pull_updates_from_remote(git_config, graph_root_location);
        if pull_result.is_err() {
            return GitActionResult::error(
                format!(
                    "Failed to pull updates from remote: {}",
                    pull_result.unwrap_err(),
                ),
                changes_pulled_from_remote,
            );
        }
    }
    let create_checkpoint_result = git_push(graph_root_location);
    if let Err(e) = create_checkpoint_result {
        return GitActionResult::error(format!("Failed to push: {e}"), changes_pulled_from_remote);
    }
    if changes_pulled_from_remote {
        if let Some(app_state) = app_state {
            println!("Updating internal state.");
            state::endpoints::refresh_internal_state(app_state);
        } else {
            println!("No application state provided, skipping internal state update.");
        }
    }
    GitActionResult::success(false, changes_pulled_from_remote)
}

pub struct GitActionResult {
    pub success: bool,
    pub commit_was_done: bool,
    pub changes_from_remote: bool,
    pub message: Option<String>,
}

pub enum GitConnect {
    ConnectedSuccessfully,
    ConnectFailed(String),
}

impl GitActionResult {
    pub fn error(message: String, changes_from_remote: bool) -> Self {
        GitActionResult {
            success: false,
            commit_was_done: false,
            message: Some(message),
            changes_from_remote,
        }
    }

    pub fn success(commit_was_done: bool, changes_from_remote: bool) -> Self {
        GitActionResult {
            message: None,
            commit_was_done,
            success: true,
            changes_from_remote,
        }
    }
}

pub fn pull_updates(
    git_config: &GitConfig,
    app_state: Data<AppState>,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &GraphChanges,
) -> GitActionResult {
    let try_updating_result =
        try_updating(git_config, graph_root_location, initiator, graph_changes);
    if let UpdateResult::Error(e) = &try_updating_result {
        return GitActionResult::error(format!("Failed to pull updates from remote: {e}"), false);
    }

    if try_updating_result == UpdateResult::HasChangedSomething {
        state::endpoints::refresh_internal_state(app_state);
    }

    GitActionResult::success(
        try_updating_result == UpdateResult::HasChangedSomething,
        try_updating_result == UpdateResult::HasChangedSomething,
    )
}

pub fn try_updating(
    git_config: &GitConfig,
    graph_root_location: &GraphRootLocation,
    initiator: CommitInitiator,
    graph_changes: &GraphChanges,
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
            last_commit: "N/A".to_string(),
        };
    }
    let has_updates_downstream = check_remote_has_incoming_updates(graph_root_location);
    let has_pending_updates_upstream = check_if_remote_has_outgoing_updates(graph_root_location);
    let has_changes = check_local_changes(graph_root_location);
    let timestamp = get_last_commit_timestamp(graph_root_location);

    GitStatus {
        enabled: config.enabled,
        is_ready: true,
        has_error: has_updates_downstream.is_error()
            || has_pending_updates_upstream.is_error()
            || has_changes.is_err(),
        has_changes: has_changes.map_err(|_| true).unwrap(),
        has_incoming_updates: has_updates_downstream.is_update_pending(),
        has_outgoing_updates: has_pending_updates_upstream.is_update_pending(),
        last_commit: timestamp,
    }
}

pub struct GitStatus {
    pub enabled: bool,
    pub is_ready: bool,
    pub has_error: bool,
    pub has_changes: bool,
    pub has_incoming_updates: bool,
    pub has_outgoing_updates: bool,
    pub last_commit: String,
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
    graph_changes: &GraphChanges,
) -> String {
    let headline = calculate_headline(commit_entity, graph_changes);
    let body = calculate_body(graph_changes);

    format!("{headline}\n\n{body}")
}

fn calculate_headline(commit_initiator: CommitInitiator, graph_changes: &GraphChanges) -> String {
    let mut sorted_changes = graph_changes
        .get_changes()
        .iter()
        .map(|change| change.change_type.description())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    sorted_changes.sort();

    let content = sorted_changes.join(", ");

    format!("{}: {}", commit_initiator.description(), content)
}

fn calculate_body(graph_changes: &GraphChanges) -> String {
    let mut sorted_changes: Vec<&GraphChange> = graph_changes.get_changes().iter().collect();
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

fn copy_recursively(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if src.is_dir() {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if file_type.is_dir() {
                copy_recursively(&src_path, &dst_path)?;
            } else {
                println!("Copying file from {:?} to {:?}", src_path, dst_path);
                std::fs::copy(&src_path, &dst_path)?;
            }
        }
    }
    Ok(())
}

pub fn setup_remote_graph(
    graph_root_location: &GraphRootLocation,
    git_graph_url: &str,
) -> GitConnect {
    let tmp_dir = graph_root_location.path.join("tmp");
    if tmp_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&tmp_dir) {
            return GitConnect::ConnectFailed(format!(
                "Failed to remove existing temp directory: {e}"
            ));
        }
    }

    if let Err(e) = std::fs::create_dir_all(&tmp_dir) {
        return GitConnect::ConnectFailed(format!("Failed to create temp directory: {e}"));
    }

    let clone_result = git_clone(
        &GraphRootLocation {
            path: tmp_dir.clone(),
        },
        git_graph_url,
    );

    if let Err(e) = clone_result {
        return GitConnect::ConnectFailed(format!("Failed to clone repository: {e}"));
    }

    let folders_to_empty = ["journals", "pages", "assets", ".git"];
    for folder in folders_to_empty {
        let folder_path = graph_root_location.path.join(folder);
        if folder_path.exists() {
            if let Err(e) = std::fs::remove_dir_all(&folder_path) {
                return GitConnect::ConnectFailed(format!("Failed to remove folder {folder}: {e}"));
            }
        }
        if let Err(e) = std::fs::create_dir_all(&folder_path) {
            return GitConnect::ConnectFailed(format!("Failed to create folder {folder}: {e}"));
        }

        if folder == ".git" {
            continue;
        }
        let gitkeep_path = folder_path.join(".gitkeep");
        if !gitkeep_path.exists() {
            if let Err(e) = std::fs::write(&gitkeep_path, "*\n!.gitkeep") {
                return GitConnect::ConnectFailed(format!(
                    "Failed to create .gitkeep file in {folder}: {e}"
                ));
            }
        }
    }

    if let Err(e) = copy_recursively(&tmp_dir, &graph_root_location.path) {
        return GitConnect::ConnectFailed(format!(
            "Failed to copy files from temp directory to graph root location: {e}"
        ));
    }

    let git_config_result = git_configure_default_options(graph_root_location);
    if let Err(e) = git_config_result {
        return GitConnect::ConnectFailed(format!("Failed to configure git options: {e}"));
    }

    if let Err(e) = std::fs::remove_dir_all(&tmp_dir) {
        return GitConnect::ConnectFailed(format!("Failed to remove temp directory: {e}"));
    }

    GitConnect::ConnectedSuccessfully
}

pub fn connect_to_empty_git_repository(
    graph_root_location: &GraphRootLocation,
    git_graph_url: &str,
) -> GitConnect {
    let folders_to_init_git_keep = ["journals", "pages", "assets"];
    for folder in folders_to_init_git_keep {
        let folder_path = graph_root_location.path.join(folder);
        let gitkeep_path = folder_path.join(".gitkeep");
        if !gitkeep_path.exists() {
            if let Err(e) = std::fs::write(&gitkeep_path, "*\n!.gitkeep") {
                return GitConnect::ConnectFailed(format!(
                    "Failed to create .gitkeep file in {folder}: {e}"
                ));
            }
        }
    }

    let git_init_result = git_init(graph_root_location);
    if let Err(e) = git_init_result {
        return GitConnect::ConnectFailed(format!("Failed to initialize git repository: {e}"));
    }

    let git_config_result = git_configure_default_options(graph_root_location);
    if let Err(e) = git_config_result {
        return GitConnect::ConnectFailed(format!("Failed to configure git options: {e}"));
    }

    let git_add_remote_result = git_add_remote(graph_root_location, git_graph_url);
    if let Err(e) = git_add_remote_result {
        return GitConnect::ConnectFailed(format!("Failed to add remote repository: {e}"));
    }

    let git_commit_result = git_commit(
        graph_root_location,
        "Initial commit: Setting up empty git repository".to_string(),
    );

    if let Err(e) = git_commit_result {
        return GitConnect::ConnectFailed(format!("Failed to commit initial changes: {e}"));
    };

    let git_push_result = git_push(graph_root_location);
    if let Err(e) = git_push_result {
        return GitConnect::ConnectFailed(format!("Failed to push initial commit: {e}"));
    }
    GitConnect::ConnectedSuccessfully
}

fn git_configure_default_options(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    git_config_user_name(graph_root_location, "Looksyk")?;
    git_config_user_email(graph_root_location, "looksyk@looksyk.looksyk")?;
    git_config_default_merge(graph_root_location)?;
    git_config_push_default(graph_root_location)?;
    git_config_default_no_edit(graph_root_location)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::io::sync_application_port::GraphChangeType;

    #[test]
    fn test_calculate_git_commit_message() {
        let message = calculate_git_commit_message(
            CommitInitiator::UserCheckpoint,
            &GraphChanges::from_iter([
                GraphChange {
                    change_type: GraphChangeType::UserPageChanged,
                    target: "Page1".to_string(),
                },
                GraphChange {
                    change_type: GraphChangeType::UserPageRenamed,
                    target: "Page2".to_string(),
                },
            ]),
        );

        assert_eq!(message, ("User Checkpoint: Wiki Page changed, Wiki Page renamed\n\nChanges:\n  * Wiki Page changed: Page1\n  * Wiki Page renamed: Page2"));
    }

    #[test]
    fn test_calculate_headline_should_destinct_events() {
        let headline = calculate_headline(
            CommitInitiator::UserCheckpoint,
            &GraphChanges::from_iter([
                GraphChange {
                    change_type: GraphChangeType::UserPageChanged,
                    target: "Page1".to_string(),
                },
                GraphChange {
                    change_type: GraphChangeType::UserPageRenamed,
                    target: "Page2".to_string(),
                },
                GraphChange {
                    change_type: GraphChangeType::UserPageChanged,
                    target: "Page3".to_string(),
                },
            ]),
        );

        assert_eq!(
            headline,
            "User Checkpoint: Wiki Page changed, Wiki Page renamed"
        );
    }
}
