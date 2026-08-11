use anstream::println;
use color_eyre::eyre::{Result, WrapErr};
use dotfiles_manager::Dfm;

use super::with_suggestions;
use crate::app::cli::SyncArgs;
use crate::app::output::{green, yellow};

/// Handle `dfm sync`.
pub fn run(ctx: &Dfm, args: SyncArgs) -> Result<()> {
    if args.dry_run {
        return dry_run(ctx);
    }

    let report = dotfiles_manager::sync::run(ctx, args.message.as_deref())
        .map_err(with_suggestions)
        .wrap_err("Sync failed")?;

    if report.committed.is_none() {
        println!("{}", yellow("   No changes to commit"));
    }

    println!("{}", green("Sync complete"));
    Ok(())
}

/// Preview `dfm sync` without staging, committing, or pushing.
fn dry_run(ctx: &Dfm) -> Result<()> {
    let changes = dotfiles_manager::sync::preview(ctx)
        .map_err(with_suggestions)
        .wrap_err("Sync preview failed")?;

    if changes.is_empty() {
        println!("{}", yellow("   No changes to commit"));
        return Ok(());
    }

    println!(
        "Dry run: would stage, commit, and push {} change(s):",
        changes.len()
    );
    for line in &changes {
        println!("   {}", line);
    }
    Ok(())
}
