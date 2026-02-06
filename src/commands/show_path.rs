use anyhow::{Result, bail};

use crate::config::GbConfig;
use crate::git;
use crate::resolve;
use crate::worktree;

/// Show the worktree path for a branch.
/// `gb --show-path <name>`
pub fn run(name: &str) -> Result<()> {
    // First check if there's an existing worktree for this branch
    if let Some(wt) = worktree::find_worktree_for_branch(name)? {
        println!("{}", wt.path.display());
        return Ok(());
    }

    // No worktree exists — compute what the path would be
    if git::branch_exists(name) {
        let config = GbConfig::load()?;
        let path = resolve::branch_to_worktree_path(&config, name);
        println!("{}", path.display());
    } else {
        bail!("fatal: branch '{}' not found", name);
    }

    Ok(())
}
