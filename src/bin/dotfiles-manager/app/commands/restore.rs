use color_eyre::eyre::{Result, WrapErr};
use dotfiles_manager::Dfm;
use dotfiles_manager::profiles::ActiveProfile;
use dotfiles_manager::registry::{ConfigRegistry, EncryptedRegistry};

use super::with_suggestions;
use crate::app::cli::RestoreArgs;
use crate::app::output::{green, is_quiet, print_section_with_summary};
use crate::app::prompt;

/// Handle `dfm restore`.
pub fn run(ctx: &Dfm, args: RestoreArgs) -> Result<()> {
    let profile = ActiveProfile::resolve(ctx, None);

    if args.dry_run {
        return dry_run(ctx, &profile, args.skip_encrypted);
    }

    let confirmed = prompt::confirm(&format!(
        "Restore will overwrite existing files at their original paths for '{}'. Continue?",
        profile
    ))?;
    if !confirmed {
        println!("Aborted, nothing was restored");
        return Ok(());
    }

    let password =
        prompt::optional_password(args.skip_encrypted, args.ask_password, "encrypted restore");

    if !is_quiet() {
        println!("Restoring...");
        println!("   Target: {}", profile);
    }

    let report = dotfiles_manager::restore::run(ctx, &profile, password.as_ref())
        .map_err(with_suggestions)
        .wrap_err("Restore failed")?;

    print_section_with_summary("Configurations", &report.configs);
    if let Some(encrypted) = &report.encrypted {
        print_section_with_summary("Encrypted configs", encrypted);
    }

    println!("{}", green("Restore complete"));
    Ok(())
}

/// Preview `dfm restore` without writing anything: lists the registry
/// entries that would be overwritten on disk, without prompting for a
/// password.
fn dry_run(ctx: &Dfm, profile: &ActiveProfile, skip_encrypted: bool) -> Result<()> {
    println!("Dry run: nothing was written");
    println!("   Target: {}", profile);

    let configs = ConfigRegistry::load_or_create(&ctx.config_registry_path())
        .map_err(with_suggestions)
        .wrap_err("Load config registry")?;
    let enabled_configs: Vec<_> = configs.get_enabled_entries().collect();
    println!("   Configurations ({}):", enabled_configs.len());
    for (_, entry) in &enabled_configs {
        println!("     {} ({})", entry.name, entry.original_path.display());
    }

    if skip_encrypted {
        println!("   Encrypted configs: skipped (--skip-encrypted)");
    } else {
        let encrypted = EncryptedRegistry::load_or_create(&ctx.encrypted_registry_path())
            .map_err(with_suggestions)
            .wrap_err("Load encrypted registry")?;
        let enabled_encrypted: Vec<_> = encrypted.get_enabled_entries().collect();
        println!("   Encrypted configs ({}):", enabled_encrypted.len());
        for (_, entry) in &enabled_encrypted {
            println!("     {} ({})", entry.name, entry.original_path.display());
        }
    }

    Ok(())
}
