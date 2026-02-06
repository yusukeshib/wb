pub const SHELL_INIT: &str = r#"# wb shell integration for bash
# Add to .bashrc: eval "$(wb init bash)"

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

# Bash completions
_wb_completions() {
  local cur prev opts branches
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"

  opts="-d -D -m -M -c -C -a -r -v -u --list --merged --no-merged --contains --no-contains --sort --show-current --show-path --edit-description --set-upstream-to --unset-upstream init"

  case "$prev" in
    -d|-D|-m|-M|-c|-C|--show-path)
      branches=$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)
      COMPREPLY=( $(compgen -W "$branches" -- "$cur") )
      return 0
      ;;
    -u|--set-upstream-to)
      branches=$(command git branch -r --format='%(refname:short)' 2>/dev/null)
      COMPREPLY=( $(compgen -W "$branches" -- "$cur") )
      return 0
      ;;
    init)
      COMPREPLY=( $(compgen -W "zsh bash fish" -- "$cur") )
      return 0
      ;;
  esac

  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
  else
    branches=$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)
    COMPREPLY=( $(compgen -W "$branches" -- "$cur") )
  fi
}
complete -F _wb_completions wb

# Prompt helper
wb_current_branch() {
  command wb --show-current 2>/dev/null
}
"#;
