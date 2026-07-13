# Changelog

All notable changes to UniWorld are documented in this file.

## [0.2.1] - 2026-07-13

### Fixed

- README Python quick-start: `display_width("Hello")` is 5, not 10; added CJK width example.

[0.2.1]: https://github.com/aguywithai/uniworld/compare/v0.2.0...v0.2.1

## [0.2.0] - 2026-05-04

### Summary

- Minor release: Unicode 17.0 UCD alignment, expanded publishing prep (GitHub Actions release workflow, CI UCD fetch), and version alignment across Rust, Python, VS Code, PowerShell, and npm metadata.

### Details

- CI can download pinned UCD conformance files before `cargo test` (see `_development/scripts/download_ucd_tests.sh`).
- Release workflow (`.github/workflows/release.yml`) runs tests on tag `v*` and publishes when registry secrets are set.

[0.2.0]: https://github.com/aguywithai/uniworld/compare/v0.1.0...v0.2.0
