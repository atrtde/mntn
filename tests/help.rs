//! Verifies clig.dev's help-text expectations: `help <subcommand>` works,
//! `--help` prints full help, and a usage error prints a short usage line
//! rather than dumping the full help text.

use std::process::Command;

fn dfm() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dfm"))
}

#[test]
fn help_subcommand_syntax_works_like_the_flag() {
    let via_subcommand = dfm().args(["help", "profile"]).output().unwrap();
    let via_flag = dfm().args(["profile", "--help"]).output().unwrap();

    assert!(via_subcommand.status.success());
    assert!(via_flag.status.success());
    assert_eq!(via_subcommand.stdout, via_flag.stdout);
}

#[test]
fn top_level_help_includes_support_links() {
    let output = dfm().arg("--help").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("docs.rs/dotfiles-manager"));
    assert!(stdout.contains("github.com/alexandretrotel/dotfiles-manager/issues"));
}

#[test]
fn missing_required_argument_prints_short_usage_not_full_help() {
    let output = dfm().arg("use").output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(stderr.contains("Usage:"));
    // Full `--help` output documents every subcommand's flags; a plain
    // usage error shouldn't dump that much detail.
    assert!(!stderr.contains("--no-input"));
}
