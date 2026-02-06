use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum GbError {
    #[error("fatal: not a git repository (or any parent up to mount point /)")]
    NotAGitRepo,

    #[error("fatal: branch '{0}' not found")]
    BranchNotFound(String),

    #[error("fatal: a branch named '{0}' already exists")]
    BranchAlreadyExists(String),

    #[error("fatal: worktree '{0}' already exists")]
    WorktreeAlreadyExists(String),

    #[error("error: the branch '{0}' is not fully merged.\nIf you are sure you want to delete it, run 'gb -D {0}'")]
    BranchNotFullyMerged(String),

    #[error("fatal: '{0}' is checked out at '{1}'")]
    BranchCheckedOut(String, String),

    #[error("error: no worktree found for branch '{0}'")]
    NoWorktreeForBranch(String),

    #[error("fatal: cannot determine current branch (not inside a worktree)")]
    NotInWorktree,

    #[error("error: invalid branch name '{0}'")]
    InvalidBranchName(String),

    #[error("{0}")]
    Git(String),

    #[error("{0}")]
    Other(String),
}
