//! CLI entry point: parses arguments and dispatches to command handlers.

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod output;
pub(crate) mod prompt;

use anstream::{eprintln, println};
use clap::{CommandFactory, Parser};
use color_eyre::eyre::Result;
use dotfiles_manager::Dfm;

use self::cli::{Cli, Command};
use self::output::Verbosity;

/// Parse CLI args and dispatch to the matching command handler.
pub fn run() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    output::set_verbosity(match (cli.quiet, cli.verbose) {
        (true, _) => Verbosity::Quiet,
        (false, true) => Verbosity::Verbose,
        (false, false) => Verbosity::Normal,
    });
    prompt::set_no_input(cli.no_input);

    let ctx = Dfm::new()?;
    if output::is_verbose() {
        eprintln!("dfm root: {}", ctx.root().display());
    }

    match cli.command {
        Some(Command::Link(args)) => commands::link::run(&ctx, args),
        Some(Command::Backup(args)) => commands::backup::run(&ctx, args),
        Some(Command::Restore(args)) => commands::restore::run(&ctx, args),
        Some(Command::Use(args)) => commands::r#use::run(&ctx, args),
        Some(Command::Profile(args)) => commands::profile::run(&ctx, args),
        Some(Command::Git(args)) => commands::git::run(&ctx, args),
        Some(Command::Status(args)) => commands::git::status(&ctx, args),
        Some(Command::Diff(args)) => commands::git::diff(&ctx, args),
        Some(Command::Sync(args)) => commands::sync::run(&ctx, args),
        Some(Command::Doctor(args)) => commands::doctor::run(&ctx, args),
        Some(Command::Secret { action }) => commands::secret::run(action),
        Some(Command::Prune(args)) => commands::prune::run(&ctx, args),
        Some(Command::Edit(args)) => commands::edit::run(&ctx, args),
        Some(Command::Completions(args)) => commands::completions::run(args.shell),
        Some(Command::Man) => commands::completions::man(),
        None => {
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
    }
}
