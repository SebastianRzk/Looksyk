use std::fmt::{Display, Formatter};
use std::pin::Pin;

pub struct GitConfigOnDisk {
    pub active: bool,
    pub git_conflict_resolution: GitConflictResolution,
}

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub enabled: bool,
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
            GitConflictResolution::KeepLocal => write!(f, "keep-local"),
            GitConflictResolution::KeepRemote => write!(f, "keep-remote"),
            GitConflictResolution::Merge => write!(f, "merge"),
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
