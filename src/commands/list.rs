use anyhow::Result;

use crate::git::{self, BranchFilter, BranchScope};
use crate::output;
use crate::worktree;

/// List branches.
/// `gb`, `gb -a`, `gb -r`, `gb -v`, `gb -vv`, `gb --list <pattern>`, etc.
pub fn run(
    all: bool,
    remotes: bool,
    verbose: u8,
    pattern: Option<&str>,
    merged: Option<&str>,
    no_merged: Option<&str>,
    contains: Option<&str>,
    no_contains: Option<&str>,
    sort: Option<&str>,
) -> Result<()> {
    let scope = if all {
        BranchScope::All
    } else if remotes {
        BranchScope::Remote
    } else {
        BranchScope::Local
    };

    let filter = BranchFilter {
        scope,
        sort: sort.map(|s| s.to_string()),
        merged: merged.map(|s| s.to_string()),
        no_merged: no_merged.map(|s| s.to_string()),
        contains: contains.map(|s| s.to_string()),
        no_contains: no_contains.map(|s| s.to_string()),
        pattern: pattern.map(|s| s.to_string()),
    };

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
    let output = output::format_branch_list(&branches, &worktrees, verbose);

    if !output.is_empty() {
        println!("{}", output);
    }

    Ok(())
}
