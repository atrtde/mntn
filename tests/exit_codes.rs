//! Verifies the `dfm` binary's exit code conventions against clig.dev:
//! 0 on success, a non-zero runtime failure code, and clap's usage-error
//! code for malformed invocations.

use std::process::Command;

fn dfm() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dfm"))
}

#[test]
fn help_exits_zero() {
    let output = dfm().arg("--help").output().unwrap();
    assert!(output.status.success());
}

#[test]
fn no_subcommand_prints_help_and_exits_zero() {
    let dir = tempfile::tempdir().unwrap();
    let output = dfm().env("DFM_ROOT", dir.path()).output().unwrap();
    assert!(output.status.success());
}

#[test]
fn unknown_subcommand_exits_with_clap_usage_error_code() {
    let output = dfm().arg("not-a-command").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn missing_required_argument_exits_with_clap_usage_error_code() {
    let output = dfm().arg("use").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn runtime_failure_exits_with_code_one() {
    let dir = tempfile::tempdir().unwrap();
    let output = dfm()
        .env("DFM_ROOT", dir.path())
        .args(["use", "nonexistent-profile"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
}
