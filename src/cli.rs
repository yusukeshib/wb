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
    pub command: Option<InitCommand>,

    /// Delete a branch (safe)
    #[arg(short = 'd', num_args = 1.., value_name = "branch")]
    pub delete: Option<Vec<String>>,

    /// Delete a branch (force)
    #[arg(short = 'D', num_args = 1.., value_name = "branch")]
    pub force_delete: Option<Vec<String>>,

    /// Move/rename a branch
    #[arg(short = 'm', num_args = 1..=2, value_name = "branch")]
    pub rename: Option<Vec<String>>,

    /// Force move/rename a branch
    #[arg(short = 'M', num_args = 1..=2, value_name = "branch")]
    pub force_rename: Option<Vec<String>>,

    /// Copy a branch
    #[arg(short = 'c', num_args = 1..=2, value_name = "branch")]
    pub copy: Option<Vec<String>>,

    /// Force copy a branch
    #[arg(short = 'C', num_args = 1..=2, value_name = "branch")]
    pub force_copy: Option<Vec<String>>,

    /// List all branches (local + remote)
    #[arg(short = 'a')]
    pub all: bool,

    /// List remote-tracking branches
    #[arg(short = 'r')]
    pub remotes: bool,

    /// Verbose output (show hash + subject)
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// List branches matching a glob pattern
    #[arg(long = "list", value_name = "pattern")]
    pub list_pattern: Option<Option<String>>,

    /// Set upstream tracking branch
    #[arg(short = 'u', long = "set-upstream-to", value_name = "upstream")]
    pub set_upstream: Option<String>,

    /// Unset upstream tracking branch
    #[arg(long = "unset-upstream")]
    pub unset_upstream: bool,

    /// Show current branch name
    #[arg(long = "show-current")]
    pub show_current: bool,

    /// Show worktree path for a branch
    #[arg(long = "show-path", value_name = "branch")]
    pub show_path: Option<String>,

    /// Only list branches merged into the named commit
    #[arg(long = "merged", value_name = "commit")]
    pub merged: Option<Option<String>>,

    /// Only list branches not merged into the named commit
    #[arg(long = "no-merged", value_name = "commit")]
    pub no_merged: Option<Option<String>>,

    /// Only list branches containing the named commit
    #[arg(long = "contains", value_name = "commit")]
    pub contains: Option<Option<String>>,

    /// Only list branches not containing the named commit
    #[arg(long = "no-contains", value_name = "commit")]
    pub no_contains: Option<Option<String>>,

    /// Sort branches by key
    #[arg(long = "sort", value_name = "key")]
    pub sort: Option<String>,

    /// Edit branch description
    #[arg(long = "edit-description")]
    pub edit_description: bool,

    /// Positional arguments: branch name and optional start-point
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum InitCommand {
    /// Initialize: clone a repo or output shell integration
    Init {
        /// URL to clone, shell name (zsh/bash/fish), or empty for in-place conversion
        target: Option<String>,

        /// Directory to clone into
        #[arg(long = "directory", short = 'd')]
        directory: Option<String>,
    },
}

impl Cli {
    /// Determine which action mode we're in.
    pub fn mode(&self) -> CliMode {
        if self.command.is_some() {
            return CliMode::Init;
        }
        if self.delete.is_some() {
            return CliMode::Delete { force: false };
        }
        if self.force_delete.is_some() {
            return CliMode::Delete { force: true };
        }
        if self.rename.is_some() {
            return CliMode::Rename { force: false };
        }
        if self.force_rename.is_some() {
            return CliMode::Rename { force: true };
        }
        if self.copy.is_some() {
            return CliMode::Copy { force: false };
        }
        if self.force_copy.is_some() {
            return CliMode::Copy { force: true };
        }
        if self.set_upstream.is_some() {
            return CliMode::SetUpstream;
        }
        if self.unset_upstream {
            return CliMode::UnsetUpstream;
        }
        if self.show_current {
            return CliMode::ShowCurrent;
        }
        if let Some(ref _name) = self.show_path {
            return CliMode::ShowPath;
        }
        if self.edit_description {
            return CliMode::EditDescription;
        }
        if !self.args.is_empty() {
            return CliMode::Create;
        }
        CliMode::List
    }
}

#[derive(Debug)]
pub enum CliMode {
    Init,
    List,
    Create,
    Delete { force: bool },
    Rename { force: bool },
    Copy { force: bool },
    SetUpstream,
    UnsetUpstream,
    ShowCurrent,
    ShowPath,
    EditDescription,
}
