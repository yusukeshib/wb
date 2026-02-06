pub mod bash;
pub mod fish;
pub mod zsh;

use anyhow::{bail, Result};

/// Output shell integration code for the given shell.
pub fn output_shell_init(shell: &str) -> Result<()> {
    match shell {
        "zsh" => {
            print!("{}", zsh::SHELL_INIT);
            Ok(())
        }
        "bash" => {
            print!("{}", bash::SHELL_INIT);
            Ok(())
        }
        "fish" => {
            print!("{}", fish::SHELL_INIT);
            Ok(())
        }
        _ => bail!("unsupported shell: '{}'. Supported: zsh, bash, fish", shell),
    }
}
