use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

/// Run a git command capturing stdout. Returns trimmed output.
pub fn run(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("failed to execute git")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        bail!("{}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Run a git command with a specific working directory.
pub fn run_in(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .context("failed to execute git")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        bail!("{}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Find the git toplevel (bare repo root or .bare directory).
pub fn find_git_dir() -> Result<PathBuf> {
    // First check if we're in a worktree managed by wb
    let git_dir = run(&["rev-parse", "--git-common-dir"])?;
    Ok(PathBuf::from(git_dir))
}

/// Find the root directory (parent of .bare).
pub fn find_root_dir() -> Result<PathBuf> {
    let git_dir = find_git_dir()?;
    // git_dir is the .bare directory; root is its parent
    if let Some(parent) = git_dir.parent() {
        Ok(parent.to_path_buf())
    } else {
        Ok(git_dir)
    }
}

/// Information about a branch from git for-each-ref.
#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
}

/// List branches using git for-each-ref.
pub fn list_branches(filter: BranchFilter) -> Result<Vec<BranchInfo>> {
    let format = "%(refname:short)\t%(HEAD)";

    let mut args = vec!["for-each-ref", "--format", format];

    let sort_arg;
    if let Some(ref sort) = filter.sort {
        sort_arg = format!("--sort={}", sort);
        args.push(&sort_arg);
    }

    let merged_arg;
    if let Some(ref merged) = filter.merged {
        merged_arg = format!("--merged={}", merged);
        args.push(&merged_arg);
    }

    let no_merged_arg;
    if let Some(ref no_merged) = filter.no_merged {
        no_merged_arg = format!("--no-merged={}", no_merged);
        args.push(&no_merged_arg);
    }

    let contains_arg;
    if let Some(ref contains) = filter.contains {
        contains_arg = format!("--contains={}", contains);
        args.push(&contains_arg);
    }

    let no_contains_arg;
    if let Some(ref no_contains) = filter.no_contains {
        no_contains_arg = format!("--no-contains={}", no_contains);
        args.push(&no_contains_arg);
    }

    args.push("refs/heads/");

    let output = run(&args)?;
    let mut branches = Vec::new();

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() < 2 {
            continue;
        }

        branches.push(BranchInfo {
            name: parts[0].to_string(),
            is_head: parts[1].trim() == "*",
            is_remote: false,
        });
    }

    // Apply glob pattern filter
    if let Some(ref pattern) = filter.pattern {
        branches.retain(|b| glob_match::glob_match(pattern, &b.name));
    }

    Ok(branches)
}

#[derive(Debug, Default)]
pub struct BranchFilter {
    pub sort: Option<String>,
    pub merged: Option<String>,
    pub no_merged: Option<String>,
    pub contains: Option<String>,
    pub no_contains: Option<String>,
    pub pattern: Option<String>,
}

/// Check if a branch exists.
pub fn branch_exists(name: &str) -> bool {
    run(&["rev-parse", "--verify", &format!("refs/heads/{}", name)]).is_ok()
}

/// Create a branch ref (without worktree).
#[allow(dead_code)]
pub fn create_branch(name: &str, start_point: Option<&str>) -> Result<()> {
    let mut args = vec!["branch", name];
    if let Some(sp) = start_point {
        args.push(sp);
    }
    run(&args)?;
    Ok(())
}

/// Delete a branch ref.
pub fn delete_branch(name: &str, force: bool) -> Result<()> {
    let flag = if force { "-D" } else { "-d" };
    run(&["branch", flag, name])?;
    Ok(())
}

/// Rename a branch ref.
pub fn rename_branch(old: &str, new: &str, force: bool) -> Result<()> {
    let flag = if force { "-M" } else { "-m" };
    run(&["branch", flag, old, new])?;
    Ok(())
}

/// Copy a branch ref.
pub fn copy_branch(old: &str, new: &str, force: bool) -> Result<()> {
    let flag = if force { "-C" } else { "-c" };
    run(&["branch", flag, old, new])?;
    Ok(())
}

/// Get the current branch name from HEAD.
#[allow(dead_code)]
pub fn current_branch() -> Result<String> {
    run(&["symbolic-ref", "--short", "HEAD"])
}
