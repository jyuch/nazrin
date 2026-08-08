# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

nazrin is a small CLI utility meant to fill in the gaps when writing batch automation
scripts on Windows. It provides zip compression/extraction, base64 encoding/decoding,
and a Windows-only `unleash` command that schedules a file for deletion on next reboot.

The user-facing command specification lives in `README.md` (written in Japanese).
When changing the CLI, update the command table and the per-command sections there too.

## Common commands

```
cargo build                                 # debug build
cargo build --release                       # release build (LTO + strip)
cargo clippy --all-targets -- -D warnings   # lint; keep this warning-free
cargo fmt                                   # format
cargo run -- <subcommand> ...               # manual check
```

There are no tests. Verify behavior by actually processing files with `cargo run` —
the easiest approach is to create a temp directory, round-trip a file, and compare
with `Get-FileHash`.

## Layout

- `src/main.rs` — CLI layer. Holds only the clap derive definitions and dispatch;
  no actual processing logic belongs here.
- `src/lib.rs` — thin crate root with nothing but module declarations.
- `src/zip.rs` — `compress()` / `expand()`. Walks with `walkdir`, reads/writes via the `zip` crate.
- `src/base64.rs` — `encode()` / `decode()`. Streams through `EncoderWriter` / `DecoderReader`
  plus `std::io::copy`, so whole files are never loaded into memory.
- `src/unleash.rs` — **Windows only**. `MoveFileExW` with `MOVEFILE_DELAY_UNTIL_REBOOT`.
- `build.rs` — embeds a Windows resource via `winres` on Windows; no-op elsewhere.
- `.cargo/config.toml` — statically links the CRT (`+crt-static`) for MSVC targets.

The crate is a lib + bin pair; the binary always goes through the library
(e.g. `nazrin::zip::...`).

## CLI structure

Built with clap v4 (derive feature). Subcommands are two levels: group + operation.

```
Cli { action: Action }
  Action::Zip    { command: ZipCommand }     -> nazrin zip compress|expand
  Action::Base64 { command: Base64Command }  -> nazrin base64 encode|decode
  Action::Unleash { target, recursive }      -> nazrin unleash   (windows only)
```

Subcommand names are derived from variant names in kebab-case, so `#[clap(name = ...)]`
is generally unnecessary. Options use `#[clap(long, short)]` and let the short flag be
derived from the first letter. Doc comments become the help text verbatim.

To add a new feature:

1. Add the implementation as a module (or a function in an existing module) under `src/`,
   shaped like `pub fn f(input: &Path, output: &Path) -> anyhow::Result<()>`.
2. Declare the module in `src/lib.rs`.
3. Add a variant to the matching child enum in `main.rs` and a match arm in that enum's
   `handle()`. If it belongs with an existing feature, put it in the existing group
   (`ZipCommand` / `Base64Command`) rather than creating a new one.
4. Update the command table and usage section in `README.md`.

## Conventions and gotchas

- **Error handling**: library code returns `anyhow::Result` and just propagates with `?`.
  Do not use `unwrap()` / `expect()`. Conversion to a process exit code is centralized in
  `handle_result()` in `main.rs`: `Ok` → 0, `Err` → print to stderr and return 1.
  clap exits with 2 on its own for parse errors. The `handle()` functions return `i32`;
  only `main()` calls `process::exit()`.
- **Windows-only code**: everything around `unleash` is gated with `#[cfg(windows)]`.
  In `main.rs` the attribute is needed on both the enum variant and the match arm.
  To avoid breaking match exhaustiveness on non-Windows builds, do not `use Action::{...}`
  to import variants — spell them out with their full path.
- **Edition 2024**: newer syntax such as let-chains (`if let Some(x) = ... && cond`)
  is in use — see `expand()` in `src/zip.rs`.
- **Path safety**: zip extraction must go through `enclosed_name()` and skip any entry
  that would escape the output directory. Do not remove this guard.
- **base64 compatibility**: output is a single line with no line breaks, and the decoder
  rejects line breaks, so it is not interoperable with `certutil -encode` output.
  If this behavior changes, fix the corresponding note in `README.md`.
- User-facing documentation is written in Japanese. `README.md` (English, essentially
  empty) and `README_jp.md` (Japanese) used to be separate; they are now consolidated
  into a single Japanese `README.md`.
