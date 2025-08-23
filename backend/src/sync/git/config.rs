use std::fmt::{Display, Formatter};
use std::sync::Mutex;

pub struct GitConfigData {
    pub config: Mutex<GitConfig>,
}

pub struct GitConfig {
    pub enabled: bool,
    pub halt_on_migration_without_internet: bool,
    pub git_sync_readyness: GitSyncReadyness,
    pub git_conflict_resolution: GitConflictResolution,
}

#[derive(Clone, Debug)]
pub enum GitConflictResolution {
    KeepLocal,
    KeepRemote,
    Merge,
}

impl Display for GitConflictResolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitConflictResolution::KeepLocal => write!(f, "ours"),
            GitConflictResolution::KeepRemote => write!(f, "theirs"),
            GitConflictResolution::Merge => write!(f, "merge"),
        }
    }
}

impl From<&String> for GitConflictResolution {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "ours" => GitConflictResolution::KeepLocal,
            "theirs" => GitConflictResolution::KeepRemote,
            "merge" => GitConflictResolution::Merge,
            _ => {
                eprintln!(
                    "Unknown git conflict resolution strategy: {value}. Defaulting to 'ours'."
                );
                GitConflictResolution::KeepLocal
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitSyncReadyness {
    ReadyAndActive,
    Disabled,
    NotReady(String),
}

pub trait GitSyncReadynessTrait {
    fn is_ready(&self) -> bool;

    fn not_ready(&self) -> bool;
}
impl GitSyncReadynessTrait for GitSyncReadyness {
    fn is_ready(&self) -> bool {
        matches!(self, GitSyncReadyness::ReadyAndActive)
    }

    fn not_ready(&self) -> bool {
        !self.is_ready()
    }
}
