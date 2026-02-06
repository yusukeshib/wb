use anyhow::{Result, bail};

use crate::git;
use crate::worktree;

/// Set upstream for a branch.
/// `wb -u <upstream> [<branch>]`
pub fn set(upstream: &str, branch: Option<&str>) -> Result<()> {
    let branch_name = match branch {
        Some(b) => b.to_string(),
        None => current_branch_from_cwd()?,
    };

    git::set_upstream(&branch_name, upstream)?;
    eprintln!(
        "branch '{}' set up to track '{}' by rebasing.",
        branch_name, upstream
    );

    Ok(())
}

/// Unset upstream for a branch.
/// `wb --unset-upstream [<branch>]`
pub fn unset(branch: Option<&str>) -> Result<()> {
    let branch_name = match branch {
        Some(b) => b.to_string(),
        None => current_branch_from_cwd()?,
    };

    git::unset_upstream(&branch_name)?;

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
