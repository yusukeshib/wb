pub const SHELL_INIT: &str = r#"# wb shell integration for fish
# Add to config.fish: wb init fish | source

function wb --wraps='command wb'
    set -l result (command wb $argv)
    set -l exit_code $status

    if string match -q '__wb_cd:*' "$result"
        set -l dir (string replace '__wb_cd:' '' "$result")
        builtin cd "$dir"
    else if test -n "$result"
        echo "$result"
    end

    return $exit_code
end

# Fish completions
complete -c wb -f

# Subcommands (only when no subcommand given yet)
complete -c wb -n '__fish_use_subcommand' -a init -d 'Initialize (shell integration or clone)'
complete -c wb -n '__fish_use_subcommand' -a list -d 'List local branches'
complete -c wb -n '__fish_use_subcommand' -a create -d 'Create a branch with worktree'
complete -c wb -n '__fish_use_subcommand' -a delete -d 'Delete branch(es) and worktrees'
complete -c wb -n '__fish_use_subcommand' -a rename -d 'Rename a branch and move worktree'
complete -c wb -n '__fish_use_subcommand' -a copy -d 'Copy a branch and create worktree'

# init subcommand
complete -c wb -n '__fish_seen_subcommand_from init' -a 'zsh bash fish'

# create/rename/copy: branch completions
complete -c wb -n '__fish_seen_subcommand_from create rename copy' -a '(command git for-each-ref --format="%(refname:short)" refs/heads/ 2>/dev/null)'

# delete: --force flag + branch completions
complete -c wb -n '__fish_seen_subcommand_from delete' -l force -d 'Force delete'
complete -c wb -n '__fish_seen_subcommand_from delete' -a '(command git for-each-ref --format="%(refname:short)" refs/heads/ 2>/dev/null)'

# Prompt helper
function wb_current_branch
    command git branch --show-current 2>/dev/null
end
"#;
