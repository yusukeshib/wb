#![allow(deprecated)]

use std::fs;
use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use predicates::prelude::*;
use tempfile::TempDir;

/// Create a normal git repo with one commit inside the given directory.
fn create_git_repo(dir: &std::path::Path) {
    Command::new("git")
        .args(["init"])
        .current_dir(dir)
        .output()
        .expect("git init failed");

    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(dir)
        .output()
        .expect("git config email failed");

    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(dir)
        .output()
        .expect("git config name failed");

    fs::write(dir.join("file.txt"), "hello\n").unwrap();
    fs::create_dir_all(dir.join("subdir")).unwrap();
    fs::write(dir.join("subdir/other.txt"), "world\n").unwrap();

    Command::new("git")
        .args(["add", "."])
        .current_dir(dir)
        .output()
        .expect("git add failed");

    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(dir)
        .output()
        .expect("git commit failed");
}

/// Get the default branch name from a repo.
fn get_branch(dir: &std::path::Path) -> String {
    let output = Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("git symbolic-ref failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn init_converts_existing_repo() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    create_git_repo(dir);
    let branch = get_branch(dir);

    // Run `wb init` inside the repo
    Command::cargo_bin("wb")
        .unwrap()
        .current_dir(dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("__wb_cd:"));

    // .git should now be a file, not a directory
    let dot_git = dir.join(".git");
    assert!(dot_git.is_file(), ".git should be a file after conversion");
    let git_content = fs::read_to_string(&dot_git).unwrap();
    assert_eq!(git_content, "gitdir: ./.bare\n");

    // .bare should be a directory
    let bare_dir = dir.join(".bare");
    assert!(bare_dir.is_dir(), ".bare should be a directory");

    // Worktree directory should exist
    let worktree_dir = dir.join(&branch);
    assert!(worktree_dir.is_dir(), "worktree directory should exist");

    // Working tree files should be in the worktree
    assert!(worktree_dir.join("file.txt").is_file());
    assert_eq!(
        fs::read_to_string(worktree_dir.join("file.txt")).unwrap(),
        "hello\n"
    );
    assert!(worktree_dir.join("subdir/other.txt").is_file());

    // Worktree admin files should include commondir (the key fix)
    let wt_admin = bare_dir.join("worktrees").join(&branch);
    assert!(wt_admin.join("commondir").exists(), "commondir must exist");
    assert!(wt_admin.join("gitdir").exists(), "gitdir must exist");
    assert!(wt_admin.join("HEAD").exists(), "HEAD must exist");

    // git status should work inside the worktree
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&worktree_dir)
        .output()
        .expect("git status failed");
    assert!(status.status.success(), "git status should succeed");
    assert!(
        String::from_utf8_lossy(&status.stdout).trim().is_empty(),
        "working tree should be clean"
    );

    // git worktree list should show the worktree
    let wt_list = Command::new("git")
        .args(["worktree", "list"])
        .current_dir(&worktree_dir)
        .output()
        .expect("git worktree list failed");
    let wt_output = String::from_utf8_lossy(&wt_list.stdout);
    assert!(
        wt_output.contains(&format!("[{}]", branch)),
        "worktree list should show the branch"
    );
}

#[test]
fn init_rejects_already_converted_repo() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    create_git_repo(dir);
    let branch = get_branch(dir);

    // First conversion
    Command::cargo_bin("wb")
        .unwrap()
        .current_dir(dir)
        .arg("init")
        .assert()
        .success();

    let worktree_dir = dir.join(&branch);
    assert!(worktree_dir.exists(), "worktree directory should exist");

    // Second conversion from worktree should fail
    Command::cargo_bin("wb")
        .unwrap()
        .current_dir(&worktree_dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already"));
}

#[test]
fn init_rejects_when_bare_dir_exists() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    create_git_repo(dir);

    // Pre-create .bare to trigger the guard
    fs::create_dir(dir.join(".bare")).unwrap();

    Command::cargo_bin("wb")
        .unwrap()
        .current_dir(dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains(".bare directory already exists"));
}
