# wb — git-branch interface backed by git-worktree

`wb` replaces the daily `git branch` / `git checkout` workflow with a worktree-based workflow. Instead of switching branches in a single working directory, each branch gets its own worktree directory.

## Bare-repo directory layout

```
~/projects/my-project/
  .bare/                    # bare git repo
  .git                      # file: "gitdir: ./.bare"
  main/                     # worktree for main
  feature--auth/            # worktree for feature/auth
```

## Install

### From crates.io

```sh
cargo install wb
```

### With Nix

```sh
# Run directly
nix run github:yusukeshib/wb

# Install to profile
nix profile install github:yusukeshib/wb

# Dev shell
nix develop github:yusukeshib/wb
```

### From source

```sh
git clone https://github.com/yusukeshib/wb.git
cd wb
cargo install --path .
```

## Shell integration

Add to your `.zshrc`:

```zsh
eval "$(wb init zsh)"
```

Also supports `bash` and `fish`.

This provides:
- A `wb()` shell function that handles `cd` into worktrees
- Tab completions
- `wb_current_branch` prompt helper

## Usage

### Clone a repo into bare-repo layout

```sh
wb init https://github.com/user/repo.git
```

### Convert an existing repo

```sh
cd my-project
wb init
```

### List branches

```sh
wb list                   # list local branches
```

### Create a branch (+ worktree, cd into it)

```sh
wb create feature-x              # creates branch + worktree, cd's into it
wb create feature-y main         # from a specific start-point
```

### Delete branches

```sh
wb delete feature-x              # safe delete (branch + worktree)
wb delete --force feature-x      # force delete
wb delete one two three          # delete multiple
```

### Rename / move

```sh
wb rename new-name old-name      # rename branch + move worktree
wb rename new-name               # rename current branch
```

### Copy

```sh
wb copy new-copy existing        # copy branch + create new worktree
wb copy new-copy                 # copy current branch
```

## Configuration

Set via `git config`:

| Key | Default | Description |
|-----|---------|-------------|
| `wb.worktreeDir` | parent of `.bare` | Base directory for worktrees |
| `wb.naming` | `flat` | Naming convention: `flat` (`/` → `--`), `nested` (`/` preserved), `prefixed` (`repo-branch`) |

## How it works

- All branch operations go through `git branch` for ref management
- Worktree operations go through `git worktree add/remove/move`
- Current branch is detected by matching `cwd` to worktree paths (not `HEAD`)
- The `__wb_cd:` protocol lets the binary signal the shell wrapper to `cd`
