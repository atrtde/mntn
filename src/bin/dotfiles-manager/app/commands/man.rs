use std::io;

use clap::CommandFactory;
use color_eyre::eyre::Result;

use crate::app::cli::Cli;

/// Handle `dfm man`: print the roff man page to stdout.
pub fn run() -> Result<()> {
    let cmd = Cli::command();
    clap_mangen::Man::new(cmd).render(&mut io::stdout())?;
    Ok(())
}
