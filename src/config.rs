use std::path::PathBuf;

use anyhow::Result;

use crate::git;

/// Naming convention for worktree directories.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum NamingConvention {
    /// `/` → `--` (e.g., `feature/auth` → `feature--auth`)
    #[default]
    Flat,
    /// `/` preserved (e.g., `feature/auth` → `feature/auth`)
    Nested,
    /// `repo-branch` (e.g., `my-project-feature-auth`)
    Prefixed,
}

/// Configuration for wb read from git config.
#[derive(Debug, Clone)]
pub struct WbConfig {
    /// Base directory for worktrees (default: parent of `.bare`).
    pub worktree_dir: PathBuf,
    /// Naming convention for worktree directories.
    pub naming: NamingConvention,
}

impl WbConfig {
    /// Load configuration from git config.
    pub fn load() -> Result<Self> {
        let root = git::find_root_dir()?;

        let worktree_dir = match git::run(&["config", "--get", "wb.worktreeDir"]) {
            Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
            _ => root,
        };

        let naming = match git::run(&["config", "--get", "wb.naming"]) {
            Ok(val) => match val.as_str() {
                "nested" => NamingConvention::Nested,
                "prefixed" => NamingConvention::Prefixed,
                _ => NamingConvention::Flat,
            },
            _ => NamingConvention::Flat,
        };

        Ok(WbConfig {
            worktree_dir,
            naming,
        })
    }
}
