use crate::state::application_state::GraphRootLocation;
use crate::sync::git::config::GitConfig;
use crate::sync::git::git_commands::RemoteUpdateResult::Error;
use crate::sync::git::io::git_command_executor::GitCommandExecutor;
use std::process::Command;

pub fn check_if_git_is_installed(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    let output = GitCommandExecutor::new("git --version", graph_root_location)
        .args(&["--version"])
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
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .env("LANG", "en_US.UTF-8")
        .env("GIT_TERMINAL_PROMPT", "0")
        .current_dir(graph_root_location.path.clone())
        .output()
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
        .args(&["fetch"])
        .execute();

    if let Err(e) = output {
        return Error(format!("Failed to execute git fetch command: {e}"));
    }

    let output = output.unwrap();

    if !output.status.success() {
        return Error("Failed to fetch updates from remote".to_string());
    }

    let output = GitCommandExecutor::new("git status", graph_root_location)
        .args(&["status", "-uno"])
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
        matches!(self, RemoteUpdateResult::Error(_))
    }
}

pub fn check_local_changes(graph_root_location: &GraphRootLocation) -> Result<bool, String> {
    let output = GitCommandExecutor::new("git status", graph_root_location)
        .args(&["status", "--porcelain"])
        .execute();

    if let Err(e) = output {
        return Err(format!("Failed to execute git status command: {e}"));
    }

    let process_output = output.unwrap();
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
        .args(&["log", "--branches", "--not", "--remotes"])
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
    use std::process::Command;

    //TODO refactor this to use GitCommandExecutor
    let mut cmd = Command::new("git");
    cmd.arg("pull");
    cmd.arg("--strategy=recursive");
    cmd.arg(format!("-X{}", config.git_conflict_resolution));
    cmd.current_dir(graph_root_location.path.clone());
    cmd.env("LANG", "en_US.UTF-8");
    cmd.env("GIT_TERMINAL_PROMPT", "0");
    println!("Executing git pull command: {cmd:?}");

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute git command: {e}"))?;
    if output.status.success() {
        Ok(())
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!(
            "Command output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Command error output: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err("Failed to pull updates from remote".to_string())
    }
}

pub fn git_commit(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    let output = GitCommandExecutor::new("git add", graph_root_location)
        .args(&["add", "."])
        .execute()?;

    if !output.status.success() {
        return Err("Failed to stage changes".to_string());
    }

    let output = GitCommandExecutor::new("git commit", graph_root_location)
        .args(&["commit", "-m", "Checkpoint commit"])
        .execute()?;

    if !output.status.success() {
        return Err("Failed to commit changes".to_string());
    }
    Ok(())
}

pub fn git_push(graph_root_location: &GraphRootLocation) -> Result<(), String> {
    let result = GitCommandExecutor::new("git push", graph_root_location)
        .args(&["push"])
        .execute();
    if let Err(e) = result {
        return Err(format!("Failed to push changes to remote: {e}"));
    }
    Ok(())
}
