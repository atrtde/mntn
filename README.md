# dotfiles-manager

dotfiles-manager (aliased as `dfm`) is built to keep your dotfiles organized, safe, and consistent across machines using profiles.

A profile is a named set of configuration choices that represents a context, like work, personal, or minimal. With profiles, you can keep multiple setups and switch between them so the right settings are active for the situation.

At a high level, dotfiles-manager helps you manage these configurations, keep them in sync, and recover them when needed.

![Demo Video](https://raw.githubusercontent.com/alexandretrotel/dotfiles-manager/main/assets/dfm.gif)

## Installation

```bash
cargo install dotfiles-manager
```

This builds and installs both binaries: `dotfiles-manager` and the shorter alias `dfm`.

Prebuilt binaries for Linux, macOS (Intel and Apple Silicon), and Windows are also available on the [Releases page](https://github.com/alexandretrotel/dotfiles-manager/releases).

## Quick Start

Setting up a new machine? Clone your existing dotfiles repo and restore in one step:

```bash
dfm link owner/repo
```

Otherwise:

```bash
dfm backup
dfm restore
dfm doctor
```

Switch profiles:

```bash
dfm profile create work --description "Work setup"
dfm use work
```

## Core Commands

| Command   | What it does                                                             |
| --------- | ------------------------------------------------------------------------- |
| `link`    | Clone a dotfiles repo into `~/.dfm` and restore it (new machine setup)    |
| `backup`  | Copy tracked configs into `~/.dfm/backup/`                                |
| `restore` | Restore configs from backup                                               |
| `doctor`  | Check registry files and config drift                                     |
| `profile` | List, create, or delete profiles                                          |
| `use`     | Switch the active profile                                                 |
| `git`     | Run any git command inside `~/.dfm`                                       |
| `status`  | Shortcut for `dfm git status`                                             |
| `diff`    | Shortcut for `dfm git diff`                                               |
| `sync`    | Commit and push changes inside `~/.dfm`                                   |
| `secret`  | Store or remove the encryption passphrase in the OS keychain              |
| `prune`   | Delete backup directories left behind by profiles that no longer exist    |
| `edit`    | Open a registry/config file (`config`, `package`, `encrypted`, `profiles`) in an editor |

**Encrypted configs:** run `dfm secret set` once you know your passphrase to persist it. Pass `--ask-password` to `backup`, `restore`, or `doctor` to type the passphrase for that run instead (bypassing the keychain) — encrypted files are still processed either way.

**Preview before you commit to it:** `backup`, `restore`, and `sync` accept `--dry-run`/`-n` to show what would happen without writing anything; `prune` accepts the same flag to list orphaned directories without deleting them.

**Scripting output:** `doctor` and `profile` accept `--json` for structured output instead of colored text.

## Global Flags

These apply to every subcommand, before or after it:

| Flag           | What it does                                                             |
| -------------- | ------------------------------------------------------------------------- |
| `-q, --quiet`  | Suppress non-essential output; warnings, errors, and final summaries still print |
| `--verbose`    | Print additional diagnostic output (e.g. the resolved dfm root)           |
| `--no-input`   | Disable all interactive prompts, for CI/automation. Confirmations default to "no"; a password prompt that can't be satisfied from the keychain fails instead of hanging |

`restore` and `profile delete` ask for confirmation before proceeding (like `prune` already did), since both are destructive — pass `--no-input` to decline automatically instead of blocking on a TTY that doesn't exist.

## Environment Variables

| Variable   | What it does                                                              |
| ---------- | --------------------------------------------------------------------------- |
| `DFM_ROOT` | Overrides the data directory dfm uses instead of the default `~/.dfm`      |
| `NO_COLOR` | Disables colored output, per the [NO_COLOR](https://no-color.org/) convention |

## Security

The encryption passphrase is never accepted as a command-line flag or an environment variable — both leak into shell history and process listings. `dfm` only ever gets it two ways: an interactive prompt (hidden, no echo), or the OS keychain via `dfm secret set`. This also means `dfm` can't be driven end-to-end non-interactively unless the passphrase is stored in the keychain first (or `--skip-encrypted` is passed); see `--no-input` below for how it fails when neither applies.

## Directory Layout

```text
~/.dfm/
├── backup/
│   ├── common/
│   │   └── encrypted/          # optional: encrypted bundle
│   └── profiles/
│       └── <name>/
│           └── encrypted/
├── profiles.json
├── .active-profile
├── config.registry.json
├── package.registry.json
└── encrypted.registry.json
```

Registry notes:
- `config.registry.json` tracks regular dotfiles and their targets.
- `package.registry.json` tracks package managers and how to export package lists.
- `encrypted.registry.json` tracks sensitive files that are stored encrypted.

## License

GNU General Public License v3.0 or later (GPL-3.0-or-later), published by the Free Software Foundation.
