use anyhow::{Result, bail};

use crate::git;
use crate::worktree;

/// Delete one or more branches and their worktrees.
/// `wb -d <name>...` or `wb -D <name>...`
pub fn run(names: &[String], force: bool) -> Result<()> {
    for name in names {
        delete_one(name, force)?;
    }
    Ok(())
}

fn delete_one(name: &str, force: bool) -> Result<()> {
    // Check if we're currently inside this worktree
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(Some(current_wt)) = worktree::find_worktree_for_path(&cwd) {
            if current_wt.branch.as_deref() == Some(name) {
                bail!("fatal: cannot delete branch '{}' while you are in its worktree", name);
            }
        }
    }

    // Remove worktree if one exists
    if let Some(wt) = worktree::find_worktree_for_branch(name)? {
        worktree::remove_worktree(&wt.path, force)?;
    }

    // Delete the branch ref
    git::delete_branch(name, force)?;

    eprintln!("Deleted branch {} (was {}).", name, short_hash(name));

    Ok(())
}

fn short_hash(branch: &str) -> String {
    git::run(&["rev-parse", "--short", &format!("refs/heads/{}", branch)])
        .unwrap_or_else(|_| "unknown".to_string())
}
