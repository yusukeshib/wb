pub const SHELL_INIT: &str = r#"# gb shell integration for zsh
# Add to .zshrc: eval "$(gb init zsh)"

gb() {
  local result
  result=$(command gb "$@")
  local exit_code=$?

  if [[ "$result" == __gb_cd:* ]]; then
    builtin cd "${result#__gb_cd:}"
  elif [[ -n "$result" ]]; then
    echo "$result"
  fi

  return $exit_code
}

# Zsh completions
_gb() {
  local -a branches

  _arguments \
    '-d[Delete branch]:*:branch:->branches' \
    '-D[Force delete branch]:*:branch:->branches' \
    '-m[Rename branch]:*:branch:->branches' \
    '-M[Force rename branch]:*:branch:->branches' \
    '-c[Copy branch]:*:branch:->branches' \
    '-C[Force copy branch]:*:branch:->branches' \
    '-a[List all branches]' \
    '-r[List remote branches]' \
    '-v[Verbose output]' \
    '-u[Set upstream]:upstream:->remotes' \
    '--set-upstream-to=[Set upstream]:upstream:->remotes' \
    '--unset-upstream[Unset upstream]' \
    '--list=[List with pattern]:pattern:' \
    '--merged=[Show merged branches]:commit:' \
    '--no-merged=[Show unmerged branches]:commit:' \
    '--contains=[Branches containing commit]:commit:' \
    '--no-contains=[Branches not containing commit]:commit:' \
    '--sort=[Sort by key]:key:(refname objectname committerdate authordate)' \
    '--show-current[Show current branch]' \
    '--show-path=[Show worktree path]:branch:->branches' \
    '--edit-description[Edit description]' \
    'init:Initialize (shell or clone)' \
    '*:branch:->branches'

  case $state in
    branches)
      local -a branch_list
      branch_list=(${(f)"$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)"})
      _describe 'branch' branch_list
      ;;
    remotes)
      local -a remote_list
      remote_list=(${(f)"$(command git branch -r --format='%(refname:short)' 2>/dev/null)"})
      _describe 'remote branch' remote_list
      ;;
  esac
}
compdef _gb gb

# Prompt helper
gb_current_branch() {
  command gb --show-current 2>/dev/null
}
"#;
