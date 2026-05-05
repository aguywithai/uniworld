# Dev notes — 2025-02-10: Rust PATH instructions and conformance setup

## Rust/Cargo on PATH (Windows 11)

Instructions added to CONTRIBUTING.md:
- Install via rustup from rustup.rs if needed; rustup adds %USERPROFILE%\.cargo\bin to user PATH.
- If already installed but cargo not found: add %USERPROFILE%\.cargo\bin to user Path in Environment Variables, then restart terminal/Cursor.
- Verify with `cargo --version` and `rustc --version`.

## Conformance test and download script

- **_development/scripts/download_ucd_tests.ps1**  
  Downloads GraphemeBreakTest.txt to _development/data/ucd/. Run from repo root. _development/data/ is in .gitignore.

- **tests/conformance_grapheme.rs**  
  Parses GraphemeBreakTest.txt (format: ÷/× then hex codepoint per token), builds string and expected boundary list, compares to grapheme_boundaries(). If the file is missing, test skips (no failure). Override path with env UNICORE_GRAPHEME_TEST.

- **data/grapheme_break.rs**  
  Added U+0903 (Devanagari SIGN VISARGA) as SpacingMark so conformance cases for × SpacingMark pass.

## Next steps

- Run `_development/scripts/download_ucd_tests.ps1`, then `cargo test grapheme_conformance_grapheme_break_test` to see pass/fail count. Fix any remaining GCB/rule gaps (e.g. Prepend, more Extend) for full pass.
- Optionally add WordBreakTest and SentenceBreakTest download + tests when word/sentence segmentation is implemented.
