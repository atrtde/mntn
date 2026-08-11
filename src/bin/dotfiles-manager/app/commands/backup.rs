use age::secrecy::SecretString;
use color_eyre::eyre::{Result, WrapErr};
use dotfiles_manager::Dfm;
use dotfiles_manager::profiles::ActiveProfile;
use dotfiles_manager::registry::{ConfigRegistry, EncryptedRegistry, PackageRegistry};

use super::with_suggestions;
use crate::app::cli::BackupArgs;
use crate::app::output::{green, is_quiet, print_section_with_summary};
use crate::app::prompt;

/// Handle `dfm backup`.
pub fn run(ctx: &Dfm, args: BackupArgs) -> Result<()> {
    let profile = ActiveProfile::resolve(ctx, args.profile.as_deref());

    if args.dry_run {
        return dry_run(ctx, &profile, args.skip_encrypted);
    }

    let password = resolve_backup_password(args.skip_encrypted, args.ask_password)?;

    if !is_quiet() {
        println!("Backing up...");
        println!("   Target: {}", profile);
    }

    let report = dotfiles_manager::backup::run(ctx, &profile, password.as_ref())
        .map_err(with_suggestions)
        .wrap_err("Backup failed")?;

    if report.repo.initialized && !is_quiet() {
        println!("Initialized git repository in {}", ctx.root().display());
    }

    print_section_with_summary("Configurations", &report.configs);
    print_section_with_summary("Package managers", &report.packages);
    if let Some(encrypted) = &report.encrypted {
        print_section_with_summary("Encrypted configs", encrypted);
    }

    println!("{}", green("Backup complete"));
    Ok(())
}

/// Preview `dfm backup` without writing anything: lists the registry
/// entries that would be processed, without prompting for a password.
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

    let packages = PackageRegistry::load_or_create(&ctx.package_registry_path())
        .map_err(with_suggestions)
        .wrap_err("Load package registry")?;
    let enabled_packages: Vec<_> = packages.get_enabled_entries().collect();
    println!("   Package managers ({}):", enabled_packages.len());
    for (_, entry) in &enabled_packages {
        println!("     {} ({})", entry.name, entry.command);
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

/// Prompt for the encryption password unless `--skip-encrypted` was passed.
fn resolve_backup_password(
    skip_encrypted: bool,
    ask_password: bool,
) -> Result<Option<SecretString>> {
    if skip_encrypted {
        return Ok(None);
    }
    prompt::resolve_password(ask_password, true)
        .wrap_err("Prompt for encryption password")
        .map(Some)
}
