pub const SHELL_INIT: &str = r#"# gb shell integration for fish
# Add to config.fish: gb init fish | source

function gb --wraps='command gb'
    set -l result (command gb $argv)
    set -l exit_code $status

    if string match -q '__gb_cd:*' "$result"
        set -l dir (string replace '__gb_cd:' '' "$result")
        builtin cd "$dir"
    else if test -n "$result"
        echo "$result"
    end

    return $exit_code
end

# Fish completions
complete -c gb -f

# Flags
complete -c gb -s d -d 'Delete branch'
complete -c gb -s D -d 'Force delete branch'
complete -c gb -s m -d 'Rename branch'
complete -c gb -s M -d 'Force rename branch'
complete -c gb -s c -d 'Copy branch'
complete -c gb -s C -d 'Force copy branch'
complete -c gb -s a -d 'List all branches'
complete -c gb -s r -d 'List remote branches'
complete -c gb -s v -d 'Verbose output'
complete -c gb -s u -d 'Set upstream'
complete -c gb -l list -d 'List with pattern'
complete -c gb -l merged -d 'Show merged branches'
complete -c gb -l no-merged -d 'Show unmerged branches'
complete -c gb -l contains -d 'Branches containing commit'
complete -c gb -l no-contains -d 'Branches not containing commit'
complete -c gb -l sort -d 'Sort by key'
complete -c gb -l show-current -d 'Show current branch'
complete -c gb -l show-path -d 'Show worktree path'
complete -c gb -l edit-description -d 'Edit description'
complete -c gb -l set-upstream-to -d 'Set upstream'
complete -c gb -l unset-upstream -d 'Unset upstream'

# Subcommand
complete -c gb -a init -d 'Initialize (shell or clone)'

# Branch name completions
complete -c gb -a '(command git for-each-ref --format="%(refname:short)" refs/heads/ 2>/dev/null)'

# Prompt helper
function gb_current_branch
    command gb --show-current 2>/dev/null
end
"#;
