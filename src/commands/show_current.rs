use anyhow::Result;

use crate::worktree;

/// Show the current branch name based on cwd → worktree mapping.
/// `wb --show-current`
pub fn run() -> Result<()> {
    let cwd = std::env::current_dir()?;

    if let Some(wt) = worktree::find_worktree_for_path(&cwd)? {
        if let Some(branch) = wt.branch {
            println!("{}", branch);
            return Ok(());
        }
    }

    // Not in a worktree — print nothing (matches git branch --show-current behavior)
    Ok(())
}
