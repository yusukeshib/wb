# gb — git-branch interface backed by git-worktree

`gb` replaces the daily `git branch` / `git checkout` workflow with a worktree-based workflow. Instead of switching branches in a single working directory, each branch gets its own worktree directory. The CLI mirrors `git branch` flags exactly.

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
eval "$(gb init zsh)"
```

Also supports `bash` and `fish`.

This provides:
- A `gb()` shell function that handles `cd` into worktrees
- Tab completions
- `gb_current_branch` prompt helper

## Usage

### Clone a repo into bare-repo layout

```sh
gb init https://github.com/user/repo.git
```

### Convert an existing repo

```sh
cd my-project
gb init
```

### Create a branch (+ worktree, cd into it)

```sh
gb feature-x              # creates branch + worktree, cd's into it
gb feature-y main         # from a specific start-point
```

### List branches

```sh
gb                        # list local branches
gb -a                     # list all (local + remote)
gb -r                     # list remote-tracking branches
gb -v                     # verbose (hash + subject)
gb -vv                    # extra verbose (+ upstream info)
gb --list 'feature/*'     # filter by glob pattern
gb --merged main          # branches merged into main
gb --contains abc123      # branches containing commit
gb --sort=-committerdate  # sort by key
```

### Delete branches

```sh
gb -d feature-x           # safe delete (branch + worktree)
gb -D feature-x           # force delete
gb -d one two three       # delete multiple
```

### Rename / move

```sh
gb -m old-name new-name   # rename branch + move worktree
gb -m new-name            # rename current branch
gb -M old new             # force rename
```

### Copy

```sh
gb -c existing new-copy   # copy branch + create new worktree
gb -C existing new-copy   # force copy
```

### Upstream tracking

```sh
gb -u origin/main         # set upstream for current branch
gb -u origin/main feature # set upstream for specific branch
gb --unset-upstream       # unset upstream
```

### Info

```sh
gb --show-current         # print current branch (from cwd)
gb --show-path feature-x  # print worktree path for branch
gb --edit-description     # edit branch description
```

## Configuration

Set via `git config`:

| Key | Default | Description |
|-----|---------|-------------|
| `gb.worktreeDir` | parent of `.bare` | Base directory for worktrees |
| `gb.naming` | `flat` | Naming convention: `flat` (`/` → `--`), `nested` (`/` preserved), `prefixed` (`repo-branch`) |

## How it works

- All branch operations go through `git branch` for ref management
- Worktree operations go through `git worktree add/remove/move`
- Current branch is detected by matching `cwd` to worktree paths (not `HEAD`)
- The `__gb_cd:` protocol lets the binary signal the shell wrapper to `cd`
