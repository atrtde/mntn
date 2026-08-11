use std::sync::OnceLock;

use colored::Colorize;
use dotfiles_manager::doctor::{DoctorReport, FixedFile, Severity};
use dotfiles_manager::{RegistryEntryStatus, SectionReport};

/// Global output verbosity, set once at startup from `--quiet`/`--verbose`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Suppress non-essential output.
    Quiet,
    /// Default verbosity.
    Normal,
    /// Print additional diagnostic output.
    Verbose,
}

static VERBOSITY: OnceLock<Verbosity> = OnceLock::new();

/// Set the process-wide verbosity. Only the first call takes effect; safe to
/// call at most once, from `app::run` right after parsing args.
pub fn set_verbosity(verbosity: Verbosity) {
    let _ = VERBOSITY.set(verbosity);
}

/// The current process-wide verbosity; `Normal` until [`set_verbosity`] runs.
fn verbosity() -> Verbosity {
    VERBOSITY.get().copied().unwrap_or(Verbosity::Normal)
}

/// Whether non-essential status output should be suppressed.
pub fn is_quiet() -> bool {
    verbosity() == Verbosity::Quiet
}

/// Whether additional diagnostic output should be printed.
pub fn is_verbose() -> bool {
    verbosity() == Verbosity::Verbose
}

/// Wrap `text` in green (success). Colorization is decided by `colored`,
/// which checks `NO_COLOR`/`CLICOLOR`/`CLICOLOR_FORCE` and whether stdout
/// is a terminal, so this degrades to plain text automatically when piped
/// or when the user opts out.
pub fn green(text: &str) -> String {
    text.green().to_string()
}

/// Wrap `text` in yellow (warning).
pub fn yellow(text: &str) -> String {
    text.yellow().to_string()
}

/// Wrap `text` in red (error).
pub fn red(text: &str) -> String {
    text.red().to_string()
}

/// Print a backup/restore section: warnings first, then one line per entry.
/// The per-entry detail is suppressed under `--quiet`; warnings and skips
/// always surface since they're diagnostics, not chatter.
fn print_section(title: &str, section: &SectionReport) {
    if !is_quiet() {
        println!("   {}: {} entries", title, section.outcomes.len());
    }

    for warning in &section.warnings {
        eprintln!("{}", yellow(&format!("     {}", warning)));
    }

    for outcome in &section.outcomes {
        match &outcome.status {
            RegistryEntryStatus::Done { note } => {
                if is_quiet() {
                    continue;
                }
                println!("     {} {}", green("✔"), outcome.label);
                if let Some(note) = note {
                    println!("       {}", note);
                }
            }
            RegistryEntryStatus::Skipped { reason } => {
                eprintln!(
                    "{}",
                    yellow(&format!(
                        "     skipped {} ({}): {}",
                        outcome.label, outcome.id, reason
                    ))
                );
            }
        }
    }
}

/// Print a section's per-entry lines followed by its succeeded/skipped
/// summary.
pub fn print_section_with_summary(title: &str, section: &SectionReport) {
    print_section(title, section);
    print_section_summary(title, section);
}

/// Print a section's succeeded/skipped counts.
fn print_section_summary(title: &str, section: &SectionReport) {
    println!(
        "   {} completed: {} succeeded, {} skipped",
        title,
        section.succeeded(),
        section.skipped()
    );
}

/// Print every validator's findings, grouped and severity-colored.
pub fn print_doctor_report(report: &DoctorReport) {
    for (name, errors) in report.results() {
        if errors.is_empty() {
            if !is_quiet() {
                println!(" {} OK", name);
            }
        } else {
            println!(" {}", name);
            for error in errors {
                let line = match error.severity {
                    Severity::Error => red(&format!(" x {}", error.message)),
                    Severity::Warning => yellow(&format!(" ! {}", error.message)),
                    Severity::Info => green(&format!(" i {}", error.message)),
                };
                println!("{}", line);
                if let Some(fix) = &error.fix_suggestion {
                    println!("{}", yellow(&format!(" Fix: {}", fix)));
                }
            }
        }
    }
}

/// Print the outcome of `dfm doctor --fix` rewriting each of dfm's own
/// registry/config files.
pub fn print_fix_report(files: &[FixedFile]) {
    println!(" Fix");
    for file in files {
        match &file.outcome {
            Ok(true) => println!(
                "{}",
                green(&format!(
                    " ✔ Rewrote {} ({})",
                    file.label,
                    file.path.display()
                ))
            ),
            Ok(false) => println!(" - {} does not exist, skipped", file.label),
            Err(e) => println!(
                "{}",
                red(&format!(
                    " x Could not rewrite {} ({}): {}",
                    file.label,
                    file.path.display(),
                    e
                ))
            ),
        }
    }
}

// `print_section`, `print_doctor_report`, and `print_fix_report` write
// directly to stdout/stderr, so meaningfully asserting on their output
// would require capturing those streams, which isn't worth the effort
// here. Only the pure color helpers below are tested.
#[cfg(test)]
mod tests {
    use super::*;

    /// `colored` decides at Display-time whether to emit ANSI codes, based
    /// on `NO_COLOR`/tty detection; under `cargo test` stdout isn't a tty,
    /// so force colorization on to actually exercise the wrapping.
    fn force_color() {
        colored::control::set_override(true);
    }

    #[test]
    fn green_contains_original_text() {
        force_color();
        assert!(green("hello").contains("hello"));
    }

    #[test]
    fn yellow_contains_original_text() {
        force_color();
        assert!(yellow("warning").contains("warning"));
    }

    #[test]
    fn red_contains_original_text() {
        force_color();
        assert!(red("error").contains("error"));
    }

    #[test]
    fn green_emits_ansi_codes_when_colorization_is_forced() {
        force_color();
        assert_ne!(green("hello"), "hello");
    }

    #[test]
    fn different_colors_produce_different_output_for_same_text() {
        force_color();
        assert_ne!(green("same"), yellow("same"));
        assert_ne!(yellow("same"), red("same"));
        assert_ne!(green("same"), red("same"));
    }

    #[test]
    fn empty_text_still_contains_empty_substring() {
        assert!(green("").contains(""));
    }
}
