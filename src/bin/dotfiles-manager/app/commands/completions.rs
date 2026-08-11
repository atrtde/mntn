use std::io;

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use color_eyre::eyre::Result;

use crate::app::cli::Cli;

/// Handle `dfm completions <shell>`: print a completion script for `shell`
/// to stdout. The script is generated for the binary target that was
/// compiled (`dfm` or `dotfiles-manager`), so completions match what the
/// user types at the prompt.
pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, env!("CARGO_BIN_NAME"), &mut io::stdout());
    Ok(())
}
