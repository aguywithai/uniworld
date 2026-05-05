# Contributing to UniWorld

UniWorld implements Unicode text handling (UAX #9, #14, #29, #15 and composite operations) in Rust with bindings for Python, JavaScript, C, and Go. Contributions are welcome.

## Source of truth

Full project specification and architecture: `_development/docs/UniWorld_PROJECT.md`.

## Development layout

- **`_development/dev_notes/`** — Create or update dev notes here before starting work.
- **`_development/scripts/`** — Auxiliary scripts (not shipped).
- **`tests/`** — Tests at repo root; included in the published source.
- **`docs/`** — Published documentation (integration guides, usage).

## Build and test

### Rust (core library)

You need the Rust toolchain (rustup and cargo) on your PATH.

**Adding Rust/Cargo to PATH on Windows 11**

1. **If Rust is not installed**: Go to [rustup.rs](https://rustup.rs/), download and run `rustup-init.exe`. Choose the default install. Rustup adds `%USERPROFILE%\.cargo\bin` to your user PATH.
2. **If Rust is installed but `cargo` is not found**: Add the Cargo bin directory to your user PATH manually:
   - Open Start, search for "Environment variables", open "Edit the system environment variables".
   - Click "Environment Variables". Under "User variables", select "Path", then "Edit" -> "New".
   - Add: `C:\Users\<YourUsername>\.cargo\bin` (replace with your username), or `%USERPROFILE%\.cargo\bin`.
   - Confirm with OK on all dialogs, then **restart your terminal or Cursor** so the new PATH is picked up.
3. **Verify**: In a new PowerShell or CMD run: `cargo --version` and `rustc --version`.

Then from the repo root:

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```

### Python (when bindings exist)

From repo root with venv activated:

```bash
.\.venv\Scripts\Activate.ps1   # PowerShell
pip install -r requirements.txt
pytest
```

## Submitting changes

- Follow the coding practices in `.cursor/rules/coding-practices.mdc` (Rust: rustfmt, Clippy; doc comments with UAX references; no Unicode in code).
- Add or update tests in `tests/` (or crate tests) as appropriate.
- Commit messages: clear and descriptive; optional prefix e.g. `[segment]`, `[bidi]`.

## Test cases and dictionaries

- **Test cases**: Conformance tests use official Unicode test files (GraphemeBreakTest.txt, etc.). For new script-specific or regression tests, add under `tests/` in the appropriate subdir (conformance, scripts, regression). Format: follow existing patterns; we can accept simple text/JSON descriptions of expected behavior for grapheme/word/line-break boundaries.
- **Dictionaries**: Thai/Lao/Khmer/Myanmar word-break dictionaries are sourced from ICU (see project doc). Contributions for additional vocabulary or locale tailoring: plain word lists or documented format; review includes native-speaker verification where possible.

## License

By contributing you agree that your contributions are under the same license as the project (MIT). Unicode and ICU data used in the project are under their respective licenses; see LICENSE and project docs.
