pub const SHELL_INIT: &str = r#"# wb shell integration for zsh
# Add to .zshrc: eval "$(wb init zsh)"

wb() {
  local result
  result=$(command wb "$@")
  local exit_code=$?

  if [[ "$result" == __wb_cd:* ]]; then
    builtin cd "${result#__wb_cd:}"
  elif [[ -n "$result" ]]; then
    echo "$result"
  fi

  return $exit_code
}

# Zsh completions
_wb() {
  local -a subcmds
  subcmds=(
    'init:Initialize (shell integration or clone)'
    'list:List local branches'
    'create:Create a branch with worktree'
    'delete:Delete branch(es) and worktrees'
    'rename:Rename a branch and move worktree'
    'copy:Copy a branch and create worktree'
  )

  if (( CURRENT == 2 )); then
    _describe 'subcommand' subcmds
    return
  fi

  case "${words[2]}" in
    init)
      if (( CURRENT == 3 )); then
        _alternative \
          'shells:shell:(zsh bash fish)' \
          'urls:url:_urls'
      elif (( CURRENT == 4 )); then
        _arguments '-d[Directory to clone into]:directory:_directories'
      fi
      ;;
    create)
      local -a branch_list
      branch_list=(${(f)"$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)"})
      _describe 'branch' branch_list
      ;;
    delete)
      _arguments \
        '--force[Force delete]' \
        '*:branch:->branches'
      if [[ $state == branches ]]; then
        local -a branch_list
        branch_list=(${(f)"$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)"})
        _describe 'branch' branch_list
      fi
      ;;
    rename|copy)
      local -a branch_list
      branch_list=(${(f)"$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)"})
      _describe 'branch' branch_list
      ;;
    list)
      ;;
  esac
}
compdef _wb wb

# Prompt helper
wb_current_branch() {
  command git branch --show-current 2>/dev/null
}
"#;
