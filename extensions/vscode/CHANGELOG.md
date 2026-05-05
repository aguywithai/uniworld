# Changelog

All notable changes to the UniWorld VS Code extension will be documented in this file.

## [0.1.0] - 2026-02-10

First stable, feature-complete release. All Phase 1-3 roadmap items done; packaging and marketplace readiness complete.

### Added
- **Extension icon**: 256x256 PNG (icon.png) for marketplace and sidebar.
- **Publishing artifacts**: LICENSE (MIT), CHANGELOG.md, .vscodeignore; package.json includes icon, keywords, galleryBanner, and full files list.

### Changed
- Version set to 0.1.0 for first stable release.
- README is marketplace-ready (features, all settings, commands, architecture).

### Fixed
- (No bug fixes in this release; packaging and docs only.)

## [0.0.10] - 2026-02-10

### Added
- **Bidi run highlighting**: LTR and RTL runs highlighted in different colours. Togglable via `uniworld.showBidiVisualization`.
- **Common-sense defaults**: Grapheme-aware cursor and delete are now **on by default** (`enableGraphemeCursor`, `enableGraphemeDelete` default to `true`).

### Changed
- Updated README with full feature and settings documentation for marketplace display.

## [0.0.9] - 2026-02-10

### Added
- **Visual bidi cursor**: Left/Right follow visual direction in RTL text (Arabic, Hebrew). Togglable via `uniworld.enableBidiVisualCursor`.
- **Line break opportunity decorations**: Subtle markers at UAX #14 break points, including dictionary-based Thai/Lao/Khmer/Myanmar. Togglable via `uniworld.showLineBreakOpportunities`.
- **Script-aware word selection**: Double-click and Ctrl+D use script-specific word boundaries. Togglable via `uniworld.enableGraphemeWordSelect`.

## [0.0.8] - 2026-02-10

### Added
- Unicode hover inspector (codepoints, display width, grapheme info on hover).
- `UniWorld: Inspect Selection` command with full breakdown.
- `UniWorld: Truncate to Display Width` command.
- Normalization commands (NFC, NFD, NFKC, NFKD).

## [0.0.7] - 2026-02-10

### Added
- Grapheme-aware cursor movement (`cursorLeftGrapheme`, `cursorRightGrapheme`).
- Grapheme-aware delete (`deleteLeftGrapheme`, `deleteRightGrapheme`).
- Settings-gated keybindings (zero impact when disabled).
- Display width and grapheme count in status bar.

## [0.0.1] - 2026-02-10

### Added
- Initial scaffold: WASM integration, status bar, extension activation.
