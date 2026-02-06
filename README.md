# wb — git-branch interface backed by git-worktree

`wb` replaces the daily `git branch` / `git checkout` workflow with a worktree-based workflow. Instead of switching branches in a single working directory, each branch gets its own worktree directory. The CLI mirrors `git branch` flags exactly.

## Bare-repo directory layout

```
~/projects/my-project/
  .bare/                    # bare git repo
  .git                      # file: "gitdir: ./.bare"
  main/                     # worktree for main
  feature--auth/            # worktree for feature/auth
```

## Install

```sh
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

### Create a branch (+ worktree, cd into it)

```sh
wb feature-x              # creates branch + worktree, cd's into it
wb feature-y main         # from a specific start-point
```

### List branches

```sh
wb                        # list local branches
wb -a                     # list all (local + remote)
wb -r                     # list remote-tracking branches
wb -v                     # verbose (hash + subject)
wb -vv                    # extra verbose (+ upstream info)
wb --list 'feature/*'     # filter by glob pattern
wb --merged main          # branches merged into main
wb --contains abc123      # branches containing commit
wb --sort=-committerdate  # sort by key
```

### Delete branches

```sh
wb -d feature-x           # safe delete (branch + worktree)
wb -D feature-x           # force delete
wb -d one two three       # delete multiple
```

### Rename / move

```sh
wb -m old-name new-name   # rename branch + move worktree
wb -m new-name            # rename current branch
wb -M old new             # force rename
```

### Copy

```sh
wb -c existing new-copy   # copy branch + create new worktree
wb -C existing new-copy   # force copy
```

### Upstream tracking

```sh
wb -u origin/main         # set upstream for current branch
wb -u origin/main feature # set upstream for specific branch
wb --unset-upstream       # unset upstream
```

### Info

```sh
wb --show-current         # print current branch (from cwd)
wb --show-path feature-x  # print worktree path for branch
wb --edit-description     # edit branch description
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
