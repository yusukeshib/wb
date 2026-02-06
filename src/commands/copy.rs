use anyhow::{Result, bail};

use crate::config::GbConfig;
use crate::git;
use crate::resolve;
use crate::worktree;

/// Copy a branch and create a new worktree.
/// `gb -c [<old>] <new>` or `gb -C [<old>] <new>`
pub fn run(names: &[String], force: bool) -> Result<()> {
    let (old_name, new_name) = match names.len() {
        1 => {
            // Copy current branch
            let current = current_branch_from_cwd()?;
            (current, names[0].clone())
        }
        2 => (names[0].clone(), names[1].clone()),
        _ => bail!("usage: gb -c [<old-branch>] <new-branch>"),
    };

    let config = GbConfig::load()?;

    // Copy the git branch ref
    git::copy_branch(&old_name, &new_name, force)?;

    // Create worktree for the new branch
    let new_path = resolve::branch_to_worktree_path(&config, &new_name);
    worktree::add_worktree(&new_path, &new_name, false, None)?;

    eprintln!(
        "Branch '{}' copied to '{}', worktree at '{}'",
        old_name,
        new_name,
        new_path.display()
    );

    println!("__gb_cd:{}", new_path.display());

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
