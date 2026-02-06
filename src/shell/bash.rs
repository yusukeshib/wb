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
  local cur prev subcmds branches
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"

  subcmds="init list create delete rename copy"

  if [[ ${COMP_CWORD} -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "$subcmds" -- "$cur") )
    return 0
  fi

  local subcmd="${COMP_WORDS[1]}"

  case "$subcmd" in
    init)
      if [[ ${COMP_CWORD} -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "zsh bash fish" -- "$cur") )
      fi
      return 0
      ;;
    create|rename|copy)
      branches=$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)
      COMPREPLY=( $(compgen -W "$branches" -- "$cur") )
      return 0
      ;;
    delete)
      if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "--force" -- "$cur") )
      else
        branches=$(command git for-each-ref --format='%(refname:short)' refs/heads/ 2>/dev/null)
        COMPREPLY=( $(compgen -W "$branches" -- "$cur") )
      fi
      return 0
      ;;
    list)
      return 0
      ;;
  esac
}
complete -F _wb_completions wb

# Prompt helper
wb_current_branch() {
  command git branch --show-current 2>/dev/null
}
"#;
