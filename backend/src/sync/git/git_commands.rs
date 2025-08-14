use crate::state::application_state::GraphRootLocation;
use crate::sync::git::config::GitConfig;
use crate::sync::git::git_commands::RemoteUpdateResult::Error;
use crate::sync::git::io::git_command_executor::GitCommandExecutor;

pub fn check_if_git_is_installed(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    let output = GitCommandExecutor::new("git --version", graph_root_location)
        .args_str(&["--version"])
        .execute()
        .map_err(|e| format!("Failed to execute git command: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Git is not installed or not found in PATH".to_string())
    }
}

pub fn check_if_git_repo_is_initialized(
    graph_root_location: &GraphRootLocation,
) -> Result<(), String> {
    let output = GitCommandExecutor::new("git rev-parse", graph_root_location)
        .args_str(&["rev-parse", "--is-inside-work-tree"])
        .execute()
        .map_err(|e| format!("Failed to execute git command: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Current directory is not a git repository".to_string())
    }
}

pub fn check_remote_has_incoming_updates(
    graph_root_location: &GraphRootLocation,
) -> RemoteUpdateResult {
    let output = GitCommandExecutor::new("git fetch", graph_root_location)
        .args_str(&["fetch"])
        .execute();

    if let Err(e) = output {
        return Error(format!("Failed to execute git fetch command: {e}"));
    }

    let output = output.unwrap();

    if !output.status.success() {
        return Error("Failed to fetch updates from remote".to_string());
    }

    let output = GitCommandExecutor::new("git status", graph_root_location)
        .args_str(&["status", "-uno"])
        .execute();

    if let Err(e) = output {
        return Error(format!("Failed to execute git status command: {e}"));
    }

    let process_output = output.unwrap();
    if !process_output.status.success() {
        return Error("Failed to check status of the repository".to_string());
    }

    let status_output = String::from_utf8_lossy(&process_output.stdout);
    let has_changes =
        status_output.contains("Your branch is behind") || status_output.contains(" have diverged");

    if has_changes {
        RemoteUpdateResult::UpdatePending
    } else {
        RemoteUpdateResult::NoUpdatePending
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoteUpdateResult {
    UpdatePending,
    NoUpdatePending,
    Error(String),
}

impl RemoteUpdateResult {
    pub fn is_update_pending(&self) -> bool {
        matches!(self, RemoteUpdateResult::UpdatePending)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Error(_))
    }
}

pub fn check_local_changes(graph_root_location: &GraphRootLocation) -> Result<bool, String> {
    let output = GitCommandExecutor::new("git status", graph_root_location)
        .args_str(&["status", "--porcelain"])
        .execute();

    if let Err(e) = output {
        return Err(format!("Failed to execute git status command: {e}"));
    }

    let process_output = output?;
    if !process_output.status.success() {
        return Err("Failed to check local changes".to_string());
    }

    let status_output = String::from_utf8_lossy(&process_output.stdout);
    Ok(!status_output.is_empty())
}

pub fn check_if_remote_has_outgoing_updates(
    graph_root_location: &GraphRootLocation,
) -> RemoteUpdateResult {
    let output = GitCommandExecutor::new("git log", graph_root_location)
        .args_str(&["log", "--branches", "--not", "--remotes"])
        .execute();

    if let Err(e) = output {
        return Error(format!("Failed to execute git log command: {e}"));
    }
    let process_output = output.unwrap();
    if !process_output.status.success() {
        return Error("Failed to check for outgoing updates".to_string());
    }
    let status_output = String::from_utf8_lossy(&process_output.stdout);
    let has_outgoing_changes = !status_output.is_empty();
    if has_outgoing_changes {
        RemoteUpdateResult::UpdatePending
    } else {
        RemoteUpdateResult::NoUpdatePending
    }
}

pub fn pull_updates_from_remote(
    config: &GitConfig,
    graph_root_location: &GraphRootLocation,
) -> Result<(), String> {
    let output = GitCommandExecutor::new("git pull", graph_root_location)
        .args(&[
            "pull".to_string(),
            "--strategy=recursive".to_string(),
            format!("-X{}", config.git_conflict_resolution),
        ])
        .execute();

    if output.is_ok() {
        Ok(())
    } else {
        Err("Failed to pull updates from remote".to_string())
    }
}

pub fn git_commit(graph_root_location: &GraphRootLocation, message: String) -> Result<(), String> {
    let output = GitCommandExecutor::new("git add", graph_root_location)
        .args_str(&["add", "."])
        .execute()?;

    if !output.status.success() {
        return Err("Failed to stage changes".to_string());
    }

    let output = GitCommandExecutor::new("git commit", graph_root_location)
        .args(&["commit".to_string(), "-m".to_string(), message])
        .execute()?;

    if !output.status.success() {
        return Err("Failed to commit changes".to_string());
    }
    Ok(())
}

pub fn git_push(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    let result = GitCommandExecutor::new("git push", graph_root_location)
        .args_str(&["push"])
        .execute();
    if let Err(e) = result {
        return Err(format!("Failed to push changes to remote: {e}"));
    }
    Ok(())
}
