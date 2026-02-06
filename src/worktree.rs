use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::git;

/// Parsed information about a single worktree.
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub head: String,
    pub branch: Option<String>,
    pub is_bare: bool,
    pub is_detached: bool,
}

/// Parse `git worktree list --porcelain` output into structured data.
pub fn list_worktrees() -> Result<Vec<WorktreeInfo>> {
    let output = git::run(&["worktree", "list", "--porcelain"])?;
    let mut worktrees = Vec::new();
    let mut current: Option<WorktreeInfo> = None;

    for line in output.lines() {
        if let Some(stripped) = line.strip_prefix("worktree ") {
            if let Some(wt) = current.take() {
                worktrees.push(wt);
            }
            current = Some(WorktreeInfo {
                path: PathBuf::from(stripped),
                head: String::new(),
                branch: None,
                is_bare: false,
                is_detached: false,
            });
        } else if let Some(stripped) = line.strip_prefix("HEAD ") {
            if let Some(ref mut wt) = current {
                wt.head = stripped.to_string();
            }
        } else if let Some(full_ref) = line.strip_prefix("branch ") {
            if let Some(ref mut wt) = current {
                // Strip refs/heads/ prefix
                let branch = full_ref.strip_prefix("refs/heads/").unwrap_or(full_ref);
                wt.branch = Some(branch.to_string());
            }
        } else if line == "bare" {
            if let Some(ref mut wt) = current {
                wt.is_bare = true;
            }
        } else if line == "detached" {
            if let Some(ref mut wt) = current {
                wt.is_detached = true;
            }
        }
    }

    if let Some(wt) = current {
        worktrees.push(wt);
    }

    Ok(worktrees)
}

/// Add a new worktree for a branch.
pub fn add_worktree(
    path: &Path,
    branch: &str,
    create_branch: bool,
    start_point: Option<&str>,
) -> Result<()> {
    let path_str = path.to_string_lossy();
    let mut args: Vec<&str> = vec!["worktree", "add"];

    if create_branch {
        args.push("-b");
        args.push(branch);
        args.push(&path_str);
        if let Some(sp) = start_point {
            args.push(sp);
        }
    } else {
        args.push(&path_str);
        args.push(branch);
    }

    git::run(&args)?;
    Ok(())
}

/// Remove a worktree.
pub fn remove_worktree(path: &Path, force: bool) -> Result<()> {
    let path_str = path.to_string_lossy();
    let mut args = vec!["worktree", "remove"];
    if force {
        args.push("--force");
    }
    args.push(&path_str);
    git::run(&args)?;
    Ok(())
}

/// Move a worktree to a new path.
pub fn move_worktree(old_path: &Path, new_path: &Path) -> Result<()> {
    let old_str = old_path.to_string_lossy();
    let new_str = new_path.to_string_lossy();
    git::run(&["worktree", "move", &old_str, &new_str])?;
    Ok(())
}

/// Find the worktree for a given branch name.
pub fn find_worktree_for_branch(branch: &str) -> Result<Option<WorktreeInfo>> {
    let worktrees = list_worktrees()?;
    Ok(worktrees
        .into_iter()
        .find(|wt| wt.branch.as_deref() == Some(branch)))
}

/// Find which worktree contains the given directory (for --show-current).
pub fn find_worktree_for_path(path: &Path) -> Result<Option<WorktreeInfo>> {
    let worktrees = list_worktrees()?;
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    // Find the worktree whose path is a prefix of the given path.
    // Choose the longest match (most specific).
    let mut best: Option<WorktreeInfo> = None;
    let mut best_len = 0;

    for wt in worktrees {
        if wt.is_bare {
            continue;
        }
        let wt_canonical = wt.path.canonicalize().unwrap_or_else(|_| wt.path.clone());
        if canonical.starts_with(&wt_canonical) {
            let len = wt_canonical.as_os_str().len();
            if len > best_len {
                best_len = len;
                best = Some(wt);
            }
        }
    }

    Ok(best)
}

/// Prune worktrees (clean up stale entries).
#[allow(dead_code)]
pub fn prune() -> Result<()> {
    git::run(&["worktree", "prune"])?;
    Ok(())
}
