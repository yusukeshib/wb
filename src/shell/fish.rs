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

# Flags
complete -c wb -s d -d 'Delete branch'
complete -c wb -s D -d 'Force delete branch'
complete -c wb -s m -d 'Rename branch'
complete -c wb -s M -d 'Force rename branch'
complete -c wb -s c -d 'Copy branch'
complete -c wb -s C -d 'Force copy branch'
complete -c wb -s a -d 'List all branches'
complete -c wb -s r -d 'List remote branches'
complete -c wb -s v -d 'Verbose output'
complete -c wb -s u -d 'Set upstream'
complete -c wb -l list -d 'List with pattern'
complete -c wb -l merged -d 'Show merged branches'
complete -c wb -l no-merged -d 'Show unmerged branches'
complete -c wb -l contains -d 'Branches containing commit'
complete -c wb -l no-contains -d 'Branches not containing commit'
complete -c wb -l sort -d 'Sort by key'
complete -c wb -l show-current -d 'Show current branch'
complete -c wb -l show-path -d 'Show worktree path'
complete -c wb -l edit-description -d 'Edit description'
complete -c wb -l set-upstream-to -d 'Set upstream'
complete -c wb -l unset-upstream -d 'Unset upstream'

# Subcommand
complete -c wb -a init -d 'Initialize (shell or clone)'

# Branch name completions
complete -c wb -a '(command git for-each-ref --format="%(refname:short)" refs/heads/ 2>/dev/null)'

# Prompt helper
function wb_current_branch
    command wb --show-current 2>/dev/null
end
"#;
