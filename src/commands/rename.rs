use anyhow::{bail, Result};

use crate::config::WbConfig;
use crate::git;
use crate::resolve;
use crate::worktree;

/// Rename a branch and move its worktree.
/// `wb rename <new> [<old>]`
pub fn run(new_name: &str, old_name: Option<&str>) -> Result<()> {
    let old_name = match old_name {
        Some(name) => name.to_string(),
        None => current_branch_from_cwd()?,
    };

    let config = WbConfig::load()?;

    // Rename the git branch ref
    git::rename_branch(&old_name, new_name, false)?;

    // Move the worktree if one exists
    if let Some(wt) = worktree::find_worktree_for_branch(new_name)? {
        // Branch ref already renamed, worktree still points to old path
        let new_path = resolve::branch_to_worktree_path(&config, new_name);
        if wt.path != new_path {
            worktree::move_worktree(&wt.path, &new_path)?;
            eprintln!(
                "Branch '{}' renamed to '{}', worktree moved to '{}'",
                old_name,
                new_name,
                new_path.display()
            );
        } else {
            eprintln!("Branch '{}' renamed to '{}'", old_name, new_name);
        }
    } else {
        eprintln!("Branch '{}' renamed to '{}'", old_name, new_name);
    }

    Ok(())
}

fn current_branch_from_cwd() -> Result<String> {
    let cwd = std::env::current_dir()?;
    if let Some(wt) = worktree::find_worktree_for_path(&cwd)? {
        if let Some(branch) = wt.branch {
            return Ok(branch);
        }
    }
    bail!("fatal: not on any branch");
}
