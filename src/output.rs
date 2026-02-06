use colored::Colorize;

use crate::git::BranchInfo;
use crate::worktree::WorktreeInfo;

/// Format branch listing output, similar to `git branch` output.
pub fn format_branch_list(
    branches: &[BranchInfo],
    worktrees: &[WorktreeInfo],
    verbose: u8,
) -> String {
    let mut lines = Vec::new();

    for branch in branches {
        let line = format_branch_line(branch, worktrees, verbose);
        lines.push(line);
    }

    lines.join("\n")
}

fn format_branch_line(branch: &BranchInfo, worktrees: &[WorktreeInfo], verbose: u8) -> String {
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

    if verbose == 0 {
        format!("{}{}{}", prefix, name, worktree_indicator)
    } else {
        let hash = branch.objectname.yellow().to_string();
        let upstream_info = if !branch.upstream_track.is_empty() {
            format!(" {}", branch.upstream_track)
        } else {
            String::new()
        };

        if verbose >= 2 {
            let upstream_name = if !branch.upstream.is_empty() {
                format!(" [{}{}]", branch.upstream.blue(), upstream_info)
            } else {
                String::new()
            };
            format!(
                "{}{}{} {} {} {}",
                prefix, name, worktree_indicator, hash, upstream_name, branch.subject
            )
        } else {
            format!(
                "{}{}{} {} {}{}",
                prefix, name, worktree_indicator, hash, branch.subject, upstream_info
            )
        }
    }
}
