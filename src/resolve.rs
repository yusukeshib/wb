use std::path::PathBuf;

use crate::config::{GbConfig, NamingConvention};

/// Convert a branch name to a worktree directory path.
pub fn branch_to_worktree_path(config: &GbConfig, branch: &str) -> PathBuf {
    let dir_name = sanitize_branch_name(branch, &config.naming);
    config.worktree_dir.join(dir_name)
}

/// Sanitize a branch name into a directory name based on naming convention.
pub fn sanitize_branch_name(name: &str, naming: &NamingConvention) -> String {
    match naming {
        NamingConvention::Flat => name.replace('/', "--"),
        NamingConvention::Nested => name.to_string(),
        NamingConvention::Prefixed => {
            // repo-branch format: replace / with -
            name.replace('/', "-")
        }
    }
}

/// Reverse: convert a directory name back to a branch name (flat convention).
#[allow(dead_code)]
pub fn dir_name_to_branch(dir_name: &str, naming: &NamingConvention) -> String {
    match naming {
        NamingConvention::Flat => dir_name.replace("--", "/"),
        NamingConvention::Nested => dir_name.to_string(),
        NamingConvention::Prefixed => dir_name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_sanitize() {
        assert_eq!(
            sanitize_branch_name("feature/auth", &NamingConvention::Flat),
            "feature--auth"
        );
        assert_eq!(
            sanitize_branch_name("main", &NamingConvention::Flat),
            "main"
        );
        assert_eq!(
            sanitize_branch_name("feat/sub/deep", &NamingConvention::Flat),
            "feat--sub--deep"
        );
    }

    #[test]
    fn test_nested_sanitize() {
        assert_eq!(
            sanitize_branch_name("feature/auth", &NamingConvention::Nested),
            "feature/auth"
        );
    }

    #[test]
    fn test_reverse_flat() {
        assert_eq!(
            dir_name_to_branch("feature--auth", &NamingConvention::Flat),
            "feature/auth"
        );
    }

    #[test]
    fn test_worktree_path() {
        let config = GbConfig {
            worktree_dir: PathBuf::from("/home/user/project"),
            naming: NamingConvention::Flat,
        };
        assert_eq!(
            branch_to_worktree_path(&config, "feature/auth"),
            PathBuf::from("/home/user/project/feature--auth")
        );
    }
}
