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
use clap::{CommandFactory, Parser};

use cli::{Cli, Command};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            Cli::command().print_help()?;
            Ok(())
        }

        Some(Command::Init { target, directory }) => match target.as_deref() {
            Some("zsh") | Some("bash") | Some("fish") => {
                shell::output_shell_init(target.as_deref().unwrap())
            }
            _ => commands::init_repo::run(target.as_deref(), directory.as_deref()),
        },

        Some(Command::List) => commands::list::run(),

        Some(Command::Create { branch, from }) => commands::create::run(&branch, from.as_deref()),

        Some(Command::Delete { branches, force }) => commands::delete::run(&branches, force),

        Some(Command::Rename { new_name, old_name }) => {
            commands::rename::run(&new_name, old_name.as_deref())
        }

        Some(Command::Copy { new_name, from }) => commands::copy::run(&new_name, from.as_deref()),
    }
}
