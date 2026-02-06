use colored::Colorize;

use crate::git::BranchInfo;
use crate::worktree::WorktreeInfo;

/// Format branch listing output, similar to `git branch` output.
pub fn format_branch_list(branches: &[BranchInfo], worktrees: &[WorktreeInfo]) -> String {
    let mut lines = Vec::new();

    for branch in branches {
        let line = format_branch_line(branch, worktrees);
        lines.push(line);
    }

    lines.join("\n")
}

fn format_branch_line(branch: &BranchInfo, worktrees: &[WorktreeInfo]) -> String {
    let prefix = if branch.is_head {
        "* ".green().to_string()
    } else {
        "  ".to_string()
    };

    let name = if branch.is_head {
        branch.name.green().to_string()
    } else if branch.is_remote {
        branch.name.red().to_string()
    } else {
        branch.name.to_string()
    };

    // Check if branch has a worktree
    let has_worktree = worktrees
        .iter()
        .any(|wt| wt.branch.as_deref() == Some(&branch.name) && !wt.is_bare);

    let worktree_indicator = if has_worktree && !branch.is_head {
        " +".cyan().to_string()
    } else {
        String::new()
    };

    format!("{}{}{}", prefix, name, worktree_indicator)
}
