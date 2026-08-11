use color_eyre::eyre::{Result, eyre};
use dotfiles_manager::Dfm;
use dotfiles_manager::profiles::{self, ProfileConfig};
use serde::Serialize;

use super::with_suggestions;
use crate::app::cli::{ProfileActions, ProfileArgs};
use crate::app::prompt;

/// A single profile, shaped for `--json` output.
#[derive(Serialize)]
struct ProfileJson {
    name: String,
    description: Option<String>,
    active: bool,
}

/// Full `dfm profile` status, shaped for `--json` output.
#[derive(Serialize)]
struct StatusJson {
    active_profile: Option<String>,
    profiles: Vec<ProfileJson>,
}

/// Handle `dfm profile` (list/create/delete).
pub fn run(ctx: &Dfm, args: ProfileArgs) -> Result<()> {
    match args.action {
        Some(ProfileActions::List) => {
            if args.json {
                print_json(ctx)
            } else {
                list(ctx)
            }
        }
        Some(ProfileActions::Create { name, description }) => {
            let created =
                profiles::create_profile(ctx, &name, description).map_err(with_suggestions)?;
            println!("Created profile '{}'", created.name);
            if let Some(desc) = created.description {
                println!("   Description: {}", desc);
            }
            println!();
            println!("Switch to this profile with: dfm use {}", created.name);
            Ok(())
        }
        Some(ProfileActions::Delete { name }) => {
            let confirmed = prompt::confirm(&format!("Delete profile '{}'?", name))?;
            if !confirmed {
                println!("Aborted, nothing was deleted");
                return Ok(());
            }

            let deleted = profiles::delete_profile(ctx, &name).map_err(with_suggestions)?;
            if let Some(dir) = deleted.retained_directory {
                println!("Profile directory exists at {}", dir.display());
                println!("The directory was NOT deleted. Remove manually if desired:");
                println!("rm -rf {}", dir.display());
            }
            println!("Deleted profile '{}'", deleted.name);
            Ok(())
        }
        None => {
            if args.json {
                return print_json(ctx);
            }
            match profiles::get_active_profile_name(ctx) {
                Some(name) => println!("Active profile: {}", name),
                None => println!("No active profile (using common only)"),
            }
            println!();
            list(ctx)?;
            println!();
            println!("Use 'dfm use <profile>' to switch profiles");
            Ok(())
        }
    }
}

/// Print the active profile and full profile list as JSON.
fn print_json(ctx: &Dfm) -> Result<()> {
    let config = ProfileConfig::load_or_default(ctx);
    let current = profiles::get_active_profile_name(ctx);

    let profiles_json = config
        .list_profiles()
        .iter()
        .map(|name| ProfileJson {
            name: name.to_string(),
            description: config.get_profile(name).and_then(|d| d.description.clone()),
            active: current.as_deref() == Some(name.as_str()),
        })
        .collect();

    let status = StatusJson {
        active_profile: current,
        profiles: profiles_json,
    };

    let rendered = serde_json::to_string_pretty(&status)
        .map_err(|e| eyre!("Failed to serialize profile status: {e}"))?;
    println!("{}", rendered);
    Ok(())
}

/// Print all known profiles, marking the active one.
fn list(ctx: &Dfm) -> Result<()> {
    let config = ProfileConfig::load_or_default(ctx);
    let profiles_list = config.list_profiles();
    let current = profiles::get_active_profile_name(ctx);

    if profiles_list.is_empty() {
        println!("No profiles configured");
        println!();
        println!("Create a profile with: dfm profile create <name>");
        return Ok(());
    }

    println!("Available profiles:");
    for name in profiles_list {
        let is_current = current.as_ref() == Some(name);
        let marker = if is_current { " ← active" } else { "" };

        match config
            .get_profile(name)
            .and_then(|d| d.description.as_ref())
        {
            Some(desc) => println!("   {} - {}{}", name, desc, marker),
            None => println!("   {}{}", name, marker),
        }
    }
    Ok(())
}
