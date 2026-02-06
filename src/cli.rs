use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "wb",
    about = "git-branch interface backed by git-worktree",
    version,
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize: clone a repo, convert existing repo, or output shell integration
    Init {
        /// URL to clone, shell name (zsh/bash/fish), or empty for in-place conversion
        target: Option<String>,

        /// Directory to clone into
        #[arg(long = "directory", short = 'd')]
        directory: Option<String>,
    },

    /// List local branches
    List,

    /// Create a branch with a worktree and cd into it
    Create {
        /// Branch name to create
        branch: String,

        /// Start point (branch or commit) to create from
        from: Option<String>,
    },

    /// Delete branch(es) and their worktrees
    Delete {
        /// Branch names to delete
        #[arg(required = true)]
        branches: Vec<String>,

        /// Force delete (like git branch -D)
        #[arg(long)]
        force: bool,
    },

    /// Rename a branch and move its worktree
    Rename {
        /// New branch name
        new_name: String,

        /// Old branch name (defaults to current branch)
        old_name: Option<String>,
    },

    /// Copy a branch and create a new worktree
    Copy {
        /// New branch name
        new_name: String,

        /// Source branch to copy from (defaults to current branch)
        from: Option<String>,
    },
}
