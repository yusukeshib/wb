use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::git;

/// Handle `gb init <target>` — either shell integration or repo clone/conversion.
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
    git::run(&[
        "clone",
        "--bare",
        url,
        &bare_dir.to_string_lossy(),
    ])?;

    // Write .git file pointing to .bare
    let git_file = dir.join(".git");
    fs::write(&git_file, "gitdir: ./.bare\n").context("failed to write .git file")?;

    // Fix remote.origin.fetch (bare clone sets it to +refs/heads/*:refs/heads/*)
    git::run_in(&bare_dir, &[
        "config",
        "remote.origin.fetch",
        "+refs/heads/*:refs/remotes/origin/*",
    ])?;

    // Determine default branch
    let default_branch = detect_default_branch(&bare_dir)?;

    // Fetch to populate remotes
    git::run_in(&bare_dir, &["fetch", "origin"])?;

    // Create worktree for default branch
    let worktree_path = dir.join(&default_branch);
    git::run_in(&bare_dir, &[
        "worktree",
        "add",
        &worktree_path.to_string_lossy(),
        &default_branch,
    ])?;

    // Output cd directive for the shell wrapper
    let canonical = worktree_path.canonicalize().unwrap_or(worktree_path);
    println!("__gb_cd:{}", canonical.display());

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

    if git_dir_output == "." || git_dir_output.ends_with("/.bare") {
        bail!("This repository is already in gb's bare-repo layout");
    }

    // Get the repo root
    let repo_root = git::run(&["rev-parse", "--show-toplevel"])?;
    let repo_root = PathBuf::from(&repo_root);

    // Get current branch
    let current_branch = git::current_branch()
        .unwrap_or_else(|_| "main".to_string());

    let dot_git = repo_root.join(".git");
    let bare_dir = repo_root.join(".bare");

    if bare_dir.exists() {
        bail!("fatal: .bare directory already exists");
    }

    // Move .git/ → .bare/
    eprintln!("Converting repository to bare-repo layout...");
    fs::rename(&dot_git, &bare_dir).context("failed to move .git to .bare")?;

    // Write .git file pointing to .bare
    fs::write(&dot_git, "gitdir: ./.bare\n").context("failed to write .git file")?;

    // Configure bare repo
    git::run_in(&bare_dir, &["config", "core.bare", "false"])?;

    // The current working tree files are already in repo_root.
    // We need to create a worktree directory and move files there.
    let worktree_path = repo_root.join(&current_branch);

    // Create worktree entry (without checking out, since files are already here)
    // First, add a worktree entry pointing to a temp location
    // Actually, we need to move the working tree files into the worktree dir

    // List all files in repo root (excluding .bare, .git, and the target worktree dir)
    let entries: Vec<_> = fs::read_dir(&repo_root)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            name_str != ".bare" && name_str != ".git" && name_str != current_branch
        })
        .collect();

    // Create worktree directory
    fs::create_dir_all(&worktree_path)?;

    // Move all working tree files into the worktree directory
    for entry in entries {
        let from = entry.path();
        let to = worktree_path.join(entry.file_name());
        fs::rename(&from, &to).with_context(|| {
            format!("failed to move {} to {}", from.display(), to.display())
        })?;
    }

    // Register the worktree with git
    // We need to set up the worktree's .git file
    let wt_git_file = worktree_path.join(".git");
    let rel_bare = pathdiff_simple(&worktree_path, &bare_dir);
    fs::write(
        &wt_git_file,
        format!("gitdir: {}/worktrees/{}\n", rel_bare, current_branch),
    )?;

    // Create worktrees dir in bare repo
    let wt_admin_dir = bare_dir.join("worktrees").join(&current_branch);
    fs::create_dir_all(&wt_admin_dir)?;

    // Write gitdir file in worktree admin dir
    let abs_worktree = worktree_path.canonicalize().unwrap_or(worktree_path.clone());
    fs::write(
        wt_admin_dir.join("gitdir"),
        format!("{}/.git\n", abs_worktree.display()),
    )?;

    // Write HEAD file
    fs::write(
        wt_admin_dir.join("HEAD"),
        format!("ref: refs/heads/{}\n", current_branch),
    )?;

    eprintln!("Converted to bare-repo layout.");
    eprintln!("Worktree for '{}' at: {}", current_branch, worktree_path.display());

    // Output cd directive
    let canonical = worktree_path.canonicalize().unwrap_or(worktree_path);
    println!("__gb_cd:{}", canonical.display());

    Ok(())
}

/// Simple relative path computation (parent → child becomes "../child" style).
fn pathdiff_simple(from: &Path, to: &Path) -> String {
    // Simple: since worktree is one level below repo root, and .bare is also one level below
    // We just use a relative path
    let from_abs = from.canonicalize().unwrap_or_else(|_| from.to_path_buf());
    let to_abs = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());

    // Count how many components we need to go up from `from` to reach common ancestor
    let from_components: Vec<_> = from_abs.components().collect();
    let to_components: Vec<_> = to_abs.components().collect();

    let common = from_components
        .iter()
        .zip(to_components.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let ups = from_components.len() - common;
    let mut result = String::new();
    for _ in 0..ups {
        result.push_str("../");
    }
    for (i, comp) in to_components[common..].iter().enumerate() {
        if i > 0 {
            result.push('/');
        }
        result.push_str(&comp.as_os_str().to_string_lossy());
    }

    if result.is_empty() {
        ".".to_string()
    } else {
        result.trim_end_matches('/').to_string()
    }
}
