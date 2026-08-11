//! CLI subcommand implementations, one module per `dfm` command.

pub mod backup;
pub mod completions;
pub mod doctor;
pub mod edit;
pub mod git;
pub mod link;
pub mod profile;
pub mod prune;
pub mod restore;
pub mod secret;
pub mod sync;
pub mod r#use;

use color_eyre::Help;
use color_eyre::eyre::eyre;
use dotfiles_manager::Error;

/// Where to report a bug or ask for help, appended to errors that have no
/// obvious next step of their own.
const BUG_REPORT_URL: &str = "https://github.com/alexandretrotel/dotfiles-manager/issues";

/// Convert a library error into an eyre report, attaching CLI suggestions
/// for the errors that have an obvious next step.
pub(crate) fn with_suggestions(e: Error) -> color_eyre::eyre::Report {
    match &e {
        Error::ProfileNotFound(name) => {
            let create = format!("Create it with: dfm profile create {}", name);
            eyre!(e)
                .suggestion(create)
                .suggestion("List available profiles with: dfm profile list")
        }
        Error::NoGitRepository { .. } => eyre!(e).suggestion("Run 'dfm backup' to initialize it."),
        Error::DataDirAlreadyExists { path } => {
            let suggestion = format!("Move or remove {} first, then retry.", path.display());
            eyre!(e).suggestion(suggestion)
        }
        Error::InvalidRepo(_) => {
            eyre!(e).suggestion("Use a URL (https://github.com/owner/repo) or `owner/repo`.")
        }
        _ => eyre!(e).suggestion(format!(
            "If this looks like a bug, report it: {}",
            BUG_REPORT_URL
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn with_suggestions_covers_every_known_variant_without_panicking() {
        let _ = with_suggestions(Error::EmptyPassword);
        let _ = with_suggestions(Error::ProfileNotFound("work".to_string()));
        let _ = with_suggestions(Error::NoGitRepository {
            path: PathBuf::from("/tmp/dfm"),
        });
        let _ = with_suggestions(Error::DataDirAlreadyExists {
            path: PathBuf::from("/tmp/dfm"),
        });
        let _ = with_suggestions(Error::InvalidRepo("nope".to_string()));
    }

    #[test]
    fn with_suggestions_preserves_the_original_error_message() {
        let report = with_suggestions(Error::EmptyPassword);
        assert_eq!(report.to_string(), "password cannot be empty");
    }
}
