use anyhow::{Result, bail};

use crate::config::WbConfig;
use crate::git;
use crate::resolve;
use crate::worktree;

/// Create a new branch with an associated worktree.
/// `wb <name> [<start-point>]`
pub fn run(name: &str, start_point: Option<&str>) -> Result<()> {
    let config = WbConfig::load()?;

    // Check if branch already exists
    if git::branch_exists(name) {
        // Branch exists — check if it already has a worktree
        if let Some(wt) = worktree::find_worktree_for_branch(name)? {
            // Already has a worktree, just cd to it
            println!("__wb_cd:{}", wt.path.display());
            return Ok(());
        }
        // Branch exists but no worktree — create worktree for it
        let wt_path = resolve::branch_to_worktree_path(&config, name);
        worktree::add_worktree(&wt_path, name, false, None)?;
        println!("__wb_cd:{}", wt_path.display());
        return Ok(());
    }

    // Create new branch + worktree
    let wt_path = resolve::branch_to_worktree_path(&config, name);

    if wt_path.exists() {
        bail!("fatal: worktree path '{}' already exists", wt_path.display());
    }

    worktree::add_worktree(&wt_path, name, true, start_point)?;
    println!("__wb_cd:{}", wt_path.display());

    Ok(())
}
