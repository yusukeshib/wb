pub const SHELL_INIT: &str = r#"# gb shell integration for bash
# Add to .bashrc: eval "$(gb init bash)"

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

# Bash completions
_gb_completions() {
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
complete -F _gb_completions gb

# Prompt helper
gb_current_branch() {
  command gb --show-current 2>/dev/null
}
"#;
