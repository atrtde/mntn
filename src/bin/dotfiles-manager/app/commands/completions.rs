use std::io;

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use color_eyre::eyre::Result;

use crate::app::cli::Cli;

/// Handle `dfm completions <shell>`: print a completion script for `shell`
/// to stdout. The script is generated for whichever binary name was
/// actually invoked (`dfm` or `dotfiles-manager`), so completions match
/// what the user types at the prompt.
pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = std::env::args()
        .next()
        .and_then(|arg0| {
            std::path::Path::new(&arg0)
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| cmd.get_name().to_string());
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}

/// Handle `dfm man`: print the roff man page to stdout.
pub fn man() -> Result<()> {
    let cmd = Cli::command();
    clap_mangen::Man::new(cmd).render(&mut io::stdout())?;
    Ok(())
}
