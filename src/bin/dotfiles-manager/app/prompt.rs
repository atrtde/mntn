use std::io::{self, Write};
use std::sync::OnceLock;

use age::secrecy::SecretString;
use anstream::eprintln;
use color_eyre::eyre::{Result, WrapErr, bail};

static NO_INPUT: OnceLock<bool> = OnceLock::new();

/// Set the process-wide `--no-input` flag. Only the first call takes
/// effect; safe to call at most once, from `app::run` right after parsing
/// args.
pub fn set_no_input(no_input: bool) {
    let _ = NO_INPUT.set(no_input);
}

/// Whether interactive prompts are disabled.
fn no_input() -> bool {
    NO_INPUT.get().copied().unwrap_or(false)
}

/// Read the encryption password from the terminal, optionally with a
/// confirmation prompt.
pub fn prompt_password(confirm: bool) -> Result<SecretString> {
    let password =
        rpassword::prompt_password("Enter encryption password: ").wrap_err("Read password")?;

    if password.is_empty() {
        bail!("Password cannot be empty");
    }

    if confirm {
        let confirmation = rpassword::prompt_password("Confirm encryption password: ")
            .wrap_err("Read password confirmation")?;
        if password != confirmation {
            bail!("Passwords do not match");
        }
    }

    Ok(SecretString::new(password.into()))
}

/// Resolve the encryption password: the stored keychain password unless
/// `ask_password` forces a prompt.
pub fn resolve_password(ask_password: bool, confirm_on_prompt: bool) -> Result<SecretString> {
    let stored = dotfiles_manager::encryption::keyring::get_stored_password();
    let had_stored = stored.is_some();
    if !ask_password && let Some(password) = stored {
        return Ok(password);
    }
    if no_input() {
        bail!(
            "A password is required but --no-input is set and none is stored; run `dfm secret set` first or drop --no-input."
        );
    }
    let password = prompt_password(confirm_on_prompt)?;
    if !had_stored {
        eprintln!(
            "Tip: run `dfm secret set` to save this password in your system keychain and skip prompts later."
        );
    }
    Ok(password)
}

/// Ask `question` as a `[y/N]` prompt. Any input other than exactly `y` or
/// `Y` (including a blank line) counts as "no" — the caller must opt in
/// explicitly, never by accident.
pub fn confirm(question: &str) -> Result<bool> {
    if no_input() {
        eprintln!("{} [y/N]: skipped (--no-input), assuming no", question);
        return Ok(false);
    }

    print!("{} [y/N]: ", question);
    io::stdout().flush().wrap_err("Flush stdout")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .wrap_err("Read confirmation")?;

    Ok(matches!(input.trim(), "y" | "Y"))
}

/// Resolve the password for an optional encrypted step. Returns `None` when
/// the step is skipped, or when resolution fails — after printing a
/// "Skipping <step>" notice.
pub fn optional_password(skip: bool, ask_password: bool, step: &str) -> Option<SecretString> {
    if skip {
        return None;
    }
    match resolve_password(ask_password, false) {
        Ok(password) => Some(password),
        Err(e) => {
            eprintln!("Skipping {}: {}", step, e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_returns_false_without_reading_stdin_when_no_input_set() {
        set_no_input(true);
        assert!(!confirm("proceed?").unwrap());
    }
}
