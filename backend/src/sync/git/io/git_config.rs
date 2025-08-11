use crate::io::fs::env::keys::LOOKSYK_CONFIG_PATH;
use crate::state::application_state::GraphRootLocation;
use crate::sync::git::config::GitSyncReadyness::{Disabled, NotReady, ReadyAndActive};
use crate::sync::git::config::{GitConfig, GitConflictResolution, GitSyncReadyness};
use crate::sync::git::git_commands::{check_if_git_is_installed, check_if_git_repo_is_initialized};
use serde::{Deserialize, Serialize};

pub fn load_git_config(graph_root_location: &GraphRootLocation) -> GitConfig {
    initialize_git_configuration(
        &load_git_config_from_disk(graph_root_location),
        &graph_root_location,
    )
}

fn load_git_config_from_disk(graph_root_location: &GraphRootLocation) -> GitConfigOnDisk {
    let git_config_file = graph_root_location
        .path
        .join(LOOKSYK_CONFIG_PATH)
        .join("git_config.json");
    if git_config_file.exists() {
        match std::fs::read_to_string(&git_config_file) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| {
                eprintln!("Failed to parse git configuration from disk. Using default.");
                disabled_config_on_disk()
            }),
            Err(e) => {
                eprintln!("Error reading git configuration file: {e}. Using default.");
                disabled_config_on_disk()
            }
        }
    } else {
        eprintln!(
            "Git configuration file not found at {:?}. Using default.",
            git_config_file
        );
        disabled_config_on_disk()
    }
}

pub fn disabled_config_on_disk() -> GitConfigOnDisk {
    GitConfigOnDisk {
        active: false,
        git_conflict_resolution: GitConflictResolution::KeepLocal.to_string(),
    }
}

pub fn save_git_config_to_disk(graph_root_location: &GraphRootLocation, config: &GitConfigOnDisk) {
    let git_config_file = graph_root_location
        .path
        .join(LOOKSYK_CONFIG_PATH)
        .join("git_config.json");
    if let Err(e) = std::fs::create_dir_all(git_config_file.parent().unwrap()) {
        eprintln!("Failed to create directory for git configuration file: {e}");
        return;
    }
    if let Err(e) = std::fs::write(&git_config_file, serde_json::to_string(config).unwrap()) {
        eprintln!("Failed to write git configuration to disk: {e}");
    }
}

#[derive(Serialize, Deserialize)]
pub struct GitConfigOnDisk {
    pub active: bool,
    pub git_conflict_resolution: String,
}

pub fn initialize_git_configuration(
    config: &GitConfigOnDisk,
    graph_root_location: &GraphRootLocation,
) -> GitConfig {
    GitConfig {
        enabled: config.active,
        git_sync_readyness: calculate_readyness(config, graph_root_location),
        git_conflict_resolution: GitConflictResolution::from(&config.git_conflict_resolution),
    }
}

fn calculate_readyness(
    config: &GitConfigOnDisk,
    graph_root_location: &GraphRootLocation,
) -> GitSyncReadyness {
    if !config.active {
        return Disabled;
    }

    let installed = check_if_git_is_installed(graph_root_location);
    if let Err(e) = &installed {
        println!("Git is not installed or not found in PATH: {e}");
        return NotReady(e.clone());
    }

    let repo_is_initialized = check_if_git_repo_is_initialized(graph_root_location);

    if let Err(e) = &repo_is_initialized {
        println!("Current directory is not a git repository: {e}");
        return NotReady(e.clone());
    }
    ReadyAndActive
}
