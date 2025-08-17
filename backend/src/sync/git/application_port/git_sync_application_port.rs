use crate::state::application_state::GraphRootLocation;
use crate::sync;
use crate::sync::git::config::{GitConfig, GitSyncReadynessTrait};
use crate::sync::git::git_services::UpdateResult;
use crate::sync::git::io::git_config;
use crate::sync::git::io::git_config::{disabled_config_on_disk, save_git_config_to_disk};
use crate::sync::io::sync_application_port::GraphChanges;
use sync::git::git_services::{create_checkpoint, push_existing_commits, try_updating};

pub fn load_git_config(graph_root_location: &GraphRootLocation) -> GitConfig {
    git_config::load_git_config(graph_root_location)
}

pub fn try_to_update_graph(
    graph_root_location: &GraphRootLocation,
    git_config: &GitConfig,
    commit_initiator: CommitInitiator,
    graph_changes: &GraphChanges,
) -> GraphChangesToClear {
    if git_config.git_sync_readyness.not_ready() {
        println!("Git configuration is not ready. Skipping update.");
        return GraphChangesToClear::None;
    }

    let result = try_updating(
        git_config,
        graph_root_location,
        commit_initiator,
        graph_changes,
    );

    if let UpdateResult::Error(e) = &result {
        println!("Failed to update graph: {e:?}");
    } else {
        println!("Graph updated successfully.");
    }

    if result == UpdateResult::HasChangedSomething {
        GraphChangesToClear::All
    } else {
        GraphChangesToClear::None
    }
}

pub fn try_to_commit_and_push(
    graph_root_location: &GraphRootLocation,
    git_config: &GitConfig,
    commit_initiator: CommitInitiator,
    graph_changes: &GraphChanges,
) -> GraphChangesToClear {
    if git_config.git_sync_readyness.not_ready() {
        println!("Git configuration is not ready. Skipping update.");
        return GraphChangesToClear::None;
    }

    let checkpoint_result = create_checkpoint(
        git_config,
        None,
        graph_root_location,
        commit_initiator,
        graph_changes,
    );
    push_existing_commits(git_config, None, graph_root_location);

    if checkpoint_result.commit_was_done {
        GraphChangesToClear::All
    } else {
        GraphChangesToClear::None
    }
}

pub fn write_default_disabled_config_to_disk(graph_root_location: &GraphRootLocation) {
    save_git_config_to_disk(graph_root_location, &disabled_config_on_disk());
}

pub enum GraphChangesToClear {
    All,
    None,
}

pub enum CommitInitiator {
    Startup,
    Shutdown,
    UserCheckpoint,
    UserUpdate,
    Migration,
}
