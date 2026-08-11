//! Verifies `dfm completions <shell>` and `dfm man` produce non-empty
//! output and exit successfully.

use std::process::Command;

fn dfm() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dfm"))
}

#[test]
fn completions_prints_a_script_for_each_supported_shell() {
    for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
        let output = dfm().args(["completions", shell]).output().unwrap();
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn man_prints_a_roff_page() {
    let output = dfm().arg("man").output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(".TH dfm"));
}
