use anyhow::Result;

use crate::git;

/// Edit the description for a branch.
/// `gb --edit-description`
/// Delegates directly to `git branch --edit-description`.
pub fn run() -> Result<()> {
    git::run_interactive(&["branch", "--edit-description"])?;
    Ok(())
}
