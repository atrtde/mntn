use std::io;

use clap::CommandFactory;
use color_eyre::eyre::{Result, WrapErr};

use crate::app::cli::Cli;

/// Handle `dfm man`: print the roff man page to stdout. The page is
/// rendered under the binary target that was compiled (`dfm` or
/// `dotfiles-manager`), not `Cli`'s fixed `#[command(name = ...)]`, so it
/// matches what the user actually types.
pub fn run() -> Result<()> {
    let mut cmd = Cli::command();
    cmd.set_bin_name(env!("CARGO_BIN_NAME"));
    cmd = cmd.name(env!("CARGO_BIN_NAME"));
    clap_mangen::Man::new(cmd)
        .render(&mut io::stdout())
        .wrap_err("Failed to render man page")
}
