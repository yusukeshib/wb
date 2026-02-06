mod cli;
mod commands;
mod config;
mod error;
mod git;
mod output;
mod resolve;
mod shell;
mod worktree;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, CliMode, InitCommand};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.mode() {
        CliMode::Init => {
            if let Some(InitCommand::Init {
                ref target,
                ref directory,
            }) = cli.command
            {
                match target.as_deref() {
                    Some("zsh") | Some("bash") | Some("fish") => {
                        shell::output_shell_init(target.as_deref().unwrap())?;
                    }
                    _ => {
                        commands::init_repo::run(target.as_deref(), directory.as_deref())?;
                    }
                }
            }
        }

        CliMode::List => {
            let pattern = cli.list_pattern.as_ref().and_then(|p| p.as_deref());

            let merged = cli.merged.as_ref().map(|m| m.as_deref().unwrap_or("HEAD"));
            let no_merged = cli
                .no_merged
                .as_ref()
                .map(|m| m.as_deref().unwrap_or("HEAD"));
            let contains = cli
                .contains
                .as_ref()
                .map(|m| m.as_deref().unwrap_or("HEAD"));
            let no_contains = cli
                .no_contains
                .as_ref()
                .map(|m| m.as_deref().unwrap_or("HEAD"));

            commands::list::run(
                cli.all,
                cli.remotes,
                cli.verbose,
                pattern,
                merged,
                no_merged,
                contains,
                no_contains,
                cli.sort.as_deref(),
            )?;
        }

        CliMode::Create => {
            let name = &cli.args[0];
            let start_point = cli.args.get(1).map(|s| s.as_str());
            commands::create::run(name, start_point)?;
        }

        CliMode::Delete { force } => {
            let names = if force {
                cli.force_delete.as_ref().unwrap()
            } else {
                cli.delete.as_ref().unwrap()
            };
            commands::delete::run(names, force)?;
        }

        CliMode::Rename { force } => {
            let names = if force {
                cli.force_rename.as_ref().unwrap()
            } else {
                cli.rename.as_ref().unwrap()
            };
            commands::rename::run(names, force)?;
        }

        CliMode::Copy { force } => {
            let names = if force {
                cli.force_copy.as_ref().unwrap()
            } else {
                cli.copy.as_ref().unwrap()
            };
            commands::copy::run(names, force)?;
        }

        CliMode::SetUpstream => {
            let upstream = cli.set_upstream.as_ref().unwrap();
            let branch = cli.args.first().map(|s| s.as_str());
            commands::upstream::set(upstream, branch)?;
        }

        CliMode::UnsetUpstream => {
            let branch = cli.args.first().map(|s| s.as_str());
            commands::upstream::unset(branch)?;
        }

        CliMode::ShowCurrent => {
            commands::show_current::run()?;
        }

        CliMode::ShowPath => {
            let name = cli.show_path.as_ref().unwrap();
            commands::show_path::run(name)?;
        }

        CliMode::EditDescription => {
            commands::edit_description::run()?;
        }
    }

    Ok(())
}
