# CLAUDE.md — Project context for Claude Code

## What is this?

`wb` is a Rust CLI tool that wraps `git branch` with a worktree-based workflow. Each branch gets its own directory instead of switching branches in-place.

## Build & test

```sh
cargo build          # compile
cargo test           # run unit tests
cargo run -- --help  # see CLI help
```

## Architecture

### Module layout

```
src/
  main.rs              — Entry point, CLI dispatch via subcommand match
  cli.rs               — Clap derive-based arg parsing (subcommand enum)
  git.rs               — All git interaction via std::process::Command
  worktree.rs          — Parse/manage worktrees via `git worktree` commands
  config.rs            — Read wb.worktreeDir, wb.naming from git config
  resolve.rs           — Branch name ↔ worktree path mapping
  output.rs            — Colored branch listing (uses `colored` crate)
  error.rs             — Error types (thiserror)
  shell/               — Shell integration output (zsh, bash, fish)
  commands/            — One module per command action
```

### Key patterns

- **Subcommand-based CLI**: `wb <command> [args]` — init, list, create, delete, rename, copy. `wb` with no args shows help.
- **`__wb_cd:` protocol**: The binary prints `__wb_cd:/path` to stdout when it wants the shell to `cd`. The shell wrapper function (from `eval "$(wb init zsh)"`) intercepts this.
- **Current branch from cwd**: Not from HEAD (meaningless in bare repo). Instead, matches cwd against worktree paths.
- **Git interaction**: All via `git::run()` / `git::run_in()` calling `std::process::Command`. No libgit2.

### Naming conventions

Branch `feature/auth` becomes directory:
- `flat` (default): `feature--auth`
- `nested`: `feature/auth`
- `prefixed`: `feature-auth`

## Implementation status

### Done
- Subcommands: init, list, create, delete, rename, copy
- `wb init <url>` — bare clone with worktree layout
- `wb init` — in-place conversion of existing repo
- Shell integration: zsh, bash, fish
- Colored output matching git-branch format
- Unit tests for resolve module
- Integration tests for init

### TODO
- Integration tests with `assert_cmd` + `tempfile` for more commands
- Edge case handling: detached HEAD, orphan branches
- Man page / --help improvements
- `wb init` in-place conversion needs more testing with complex repos
- Consider adding `wb prune` to clean stale worktrees
