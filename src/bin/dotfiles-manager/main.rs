pub(crate) mod app;

use color_eyre::eyre::Result;

/// Entry point for the `dotfiles-manager` binary; delegates to the app runner.
///
/// Exit codes: `0` on success, `1` when a command runs but fails (an `Err`
/// from `app::run`, reported via `color_eyre`), `2` on malformed
/// invocations (unknown subcommand, missing required argument — handled by
/// `clap` before `app::run` is ever called).
fn main() -> Result<()> {
    app::run()
}
