use anyhow::{bail, Result};

use crate::config::WbConfig;
use crate::git;
use crate::resolve;
use crate::worktree;

/// Copy a branch and create a new worktree.
/// `wb copy <new> [<from>]`
pub fn run(new_name: &str, from: Option<&str>) -> Result<()> {
    let old_name = match from {
        Some(name) => name.to_string(),
        None => current_branch_from_cwd()?,
    };

    let config = WbConfig::load()?;

    // Copy the git branch ref
    git::copy_branch(&old_name, new_name, false)?;

    // Create worktree for the new branch
    let new_path = resolve::branch_to_worktree_path(&config, new_name);
    worktree::add_worktree(&new_path, new_name, false, None)?;

    eprintln!(
        "Branch '{}' copied to '{}', worktree at '{}'",
        old_name,
        new_name,
        new_path.display()
    );

    println!("__wb_cd:{}", new_path.display());

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
