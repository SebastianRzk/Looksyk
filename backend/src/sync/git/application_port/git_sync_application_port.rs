use crate::state::application_state::GraphRootLocation;
use crate::sync;
use crate::sync::git::config::{GitConfig, GitSyncReadynessTrait};
use crate::sync::git::git_services::UpdateResult;
use crate::sync::git::io::git_config;
use crate::sync::git::io::git_config::{disabled_config_on_disk, save_git_config_to_disk};
use sync::git::git_services::{create_checkpoint, push_existing_commits, try_updating};

pub fn load_git_config(graph_root_location: &GraphRootLocation) -> GitConfig {
    git_config::load_git_config(graph_root_location)
}

pub fn try_to_update_graph(graph_root_location: &GraphRootLocation, git_config: &GitConfig) {
    if git_config.git_sync_readyness.not_ready() {
        println!("Git configuration is not ready. Skipping update.");
        return;
    }

    let result = try_updating(git_config, graph_root_location);

    if let UpdateResult::Error(e) = result {
        println!("Failed to update graph: {e:?}");
    } else {
        println!("Graph updated successfully.");
    }
}

pub fn try_to_commit_and_push(graph_root_location: &GraphRootLocation, git_config: &GitConfig) {
    if git_config.git_sync_readyness.not_ready() {
        println!("Git configuration is not ready. Skipping update.");
        return;
    }

    create_checkpoint(git_config, None, graph_root_location);
    push_existing_commits(git_config, None, graph_root_location);
}

pub fn write_default_disabled_config_to_disk(graph_root_location: &GraphRootLocation) {
    save_git_config_to_disk(graph_root_location, &disabled_config_on_disk());
}
