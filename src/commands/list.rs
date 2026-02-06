use anyhow::Result;

use crate::git::{self, BranchFilter};
use crate::output;
use crate::worktree;

/// List local branches.
pub fn run() -> Result<()> {
    let filter = BranchFilter::default();

    // Detect current branch from cwd
    let mut branches = git::list_branches(filter)?;

    // Mark which branch is "current" based on cwd → worktree
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(Some(wt)) = worktree::find_worktree_for_path(&cwd) {
            if let Some(ref branch_name) = wt.branch {
                for b in &mut branches {
                    b.is_head = b.name == *branch_name;
                }
            }
        }
    }

    let worktrees = worktree::list_worktrees().unwrap_or_default();
    let output = output::format_branch_list(&branches, &worktrees);

    if !output.is_empty() {
        println!("{}", output);
    }

    Ok(())
}
