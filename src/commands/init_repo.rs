use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::git;

/// Handle `wb init <target>` — either shell integration or repo clone/conversion.
pub fn run(target: Option<&str>, directory: Option<&str>) -> Result<()> {
    match target {
        Some("zsh") | Some("bash") | Some("fish") => {
            // Handled by shell module, not here
            unreachable!("shell init is handled by the shell module");
        }
        Some(url) => clone_bare(url, directory),
        None => convert_existing(),
    }
}

/// Clone a repository into the bare-repo + worktree layout.
fn clone_bare(url: &str, directory: Option<&str>) -> Result<()> {
    // Determine target directory
    let dir = if let Some(d) = directory {
        PathBuf::from(d)
    } else {
        // Derive from URL: last component, strip .git suffix
        let name = url
            .rsplit('/')
            .next()
            .unwrap_or("repo")
            .strip_suffix(".git")
            .unwrap_or(url.rsplit('/').next().unwrap_or("repo"));
        PathBuf::from(name)
    };

    if dir.exists() {
        bail!("fatal: destination path '{}' already exists", dir.display());
    }

    // Create directory structure
    fs::create_dir_all(&dir).context("failed to create directory")?;

    let bare_dir = dir.join(".bare");

    // Clone as bare repo
    eprintln!("Cloning into bare repository '{}'...", bare_dir.display());
    git::run(&["clone", "--bare", url, &bare_dir.to_string_lossy()])?;

    // Write .git file pointing to .bare
    let git_file = dir.join(".git");
    fs::write(&git_file, "gitdir: ./.bare\n").context("failed to write .git file")?;

    // Fix remote.origin.fetch (bare clone sets it to +refs/heads/*:refs/heads/*)
    git::run_in(
        &bare_dir,
        &[
            "config",
            "remote.origin.fetch",
            "+refs/heads/*:refs/remotes/origin/*",
        ],
    )?;

    // Determine default branch
    let default_branch = detect_default_branch(&bare_dir)?;

    // Fetch to populate remotes
    git::run_in(&bare_dir, &["fetch", "origin"])?;

    // Create worktree for default branch
    let worktree_path = dir.join(&default_branch);
    git::run_in(
        &bare_dir,
        &[
            "worktree",
            "add",
            &worktree_path.to_string_lossy(),
            &default_branch,
        ],
    )?;

    // Output cd directive for the shell wrapper
    let canonical = worktree_path.canonicalize().unwrap_or(worktree_path);
    println!("__wb_cd:{}", canonical.display());

    Ok(())
}

/// Detect the default branch from a bare repo.
fn detect_default_branch(bare_dir: &Path) -> Result<String> {
    // Try symbolic-ref HEAD
    if let Ok(head_ref) = git::run_in(bare_dir, &["symbolic-ref", "--short", "HEAD"]) {
        if !head_ref.is_empty() {
            return Ok(head_ref);
        }
    }
    // Fallback
    Ok("main".to_string())
}

/// Convert an existing (non-bare) repo to bare-repo + worktree layout.
fn convert_existing() -> Result<()> {
    // Must be inside a git repo
    let git_dir_output = git::run(&["rev-parse", "--git-dir"])?;

    if git_dir_output == "."
        || git_dir_output.ends_with("/.bare")
        || git_dir_output.contains("/.bare/")
    {
        bail!("This repository is already in wb's bare-repo layout");
    }

    // Get the repo root
    let repo_root = git::run(&["rev-parse", "--show-toplevel"])?;
    let repo_root = PathBuf::from(&repo_root);

    // Get current branch
    let current_branch = git::current_branch().unwrap_or_else(|_| "main".to_string());

    let dot_git = repo_root.join(".git");
    let bare_dir = repo_root.join(".bare");

    if bare_dir.exists() {
        bail!("fatal: .bare directory already exists");
    }

    // Collect working tree files before moving .git
    let entries: Vec<_> = fs::read_dir(&repo_root)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            name_str != ".git" && name_str != ".bare"
        })
        .collect();

    // Move .git/ → .bare/
    eprintln!("Converting repository to bare-repo layout...");
    fs::rename(&dot_git, &bare_dir).context("failed to move .git to .bare")?;

    // Write .git file pointing to .bare
    fs::write(&dot_git, "gitdir: ./.bare\n").context("failed to write .git file")?;

    // Mark as bare so the root directory isn't treated as a working tree.
    // Worktree commands work fine with bare repos (same as clone_bare path).
    git::run_in(&bare_dir, &["config", "core.bare", "true"])?;

    // Use `git worktree add` to properly create the worktree with all admin files,
    // then move existing working tree files into it.
    let worktree_path = repo_root.join(&current_branch);

    // Detach HEAD in the bare repo so the branch isn't "checked out" there
    let head_commit = git::run_in(&bare_dir, &["rev-parse", "HEAD"])?;
    git::run_in(
        &bare_dir,
        &["update-ref", "--no-deref", "HEAD", &head_commit],
    )?;

    // Prune stale worktree entries (e.g. from a previous failed conversion)
    git::run_in(&bare_dir, &["worktree", "prune"])?;

    // Create the worktree via git (sets up .git file, commondir, index, etc.)
    git::run_in(
        &bare_dir,
        &[
            "worktree",
            "add",
            &worktree_path.to_string_lossy(),
            &current_branch,
        ],
    )?;

    // Remove the freshly checked-out files from the worktree (we'll move ours in)
    let checkout_entries: Vec<_> = fs::read_dir(&worktree_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            name_str != ".git"
        })
        .collect();
    for entry in checkout_entries {
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    // Move original working tree files into the worktree directory
    for entry in entries {
        let from = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // Skip the worktree directory itself if it somehow got collected
        if name_str == current_branch {
            continue;
        }
        let to = worktree_path.join(&*name_str);
        fs::rename(&from, &to)
            .with_context(|| format!("failed to move {} to {}", from.display(), to.display()))?;
    }

    eprintln!("Converted to bare-repo layout.");
    eprintln!(
        "Worktree for '{}' at: {}",
        current_branch,
        worktree_path.display()
    );

    // Output cd directive
    let canonical = worktree_path.canonicalize().unwrap_or(worktree_path);
    println!("__wb_cd:{}", canonical.display());

    Ok(())
}
