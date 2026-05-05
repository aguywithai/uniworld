# Dev Notes: VS Code Extension Phase 3 Implementation

**Date**: 2026-02-12
**Focus**: Phase 3 advanced features for the UniWorld VS Code extension

## Summary

Implemented three of four Phase 3 features: visual bidi cursor, line break
opportunity decorations, and grapheme-aware word selection. All three are
wired into user-facing settings and declarative keybindings.

## Work Completed

### 1. New WASM Bindings (src/wasm_bindings.rs)

Added four new WASM-exported functions to support Phase 3:

- `bidi_levels(text)` -> `Vec<u8>`: resolved embedding level per character
  (UAX #9). Level 0 = LTR, odd levels = RTL.
- `bidi_paragraph_level(text)` -> `u8`: paragraph base direction (0 or 1).
- `bidi_reorder(text)` -> `Vec<usize>`: visual reorder indices.
- `line_break_opportunities(text)` -> `Vec<u32>`: flat pairs of
  `[byte_offset, action]` for Mandatory (0) and Allowed (1) breaks.
  Uses dictionary-based segmentation for Thai/Lao/Khmer/Myanmar.

### 2. WASM Rebuild

- Set `rustup default stable` (Rust 1.93.0) to resolve PATH issues.
- Built with `cargo build --target wasm32-unknown-unknown --release --features wasm`.
- Generated JS glue with `wasm-bindgen` CLI (v0.2.108, matching wasm-bindgen
  crate version).
- Output: `extensions/vscode/wasm/` (4.7 MB .wasm + JS + .d.ts files).

### 3. Visual Bidi Cursor (Phase 3)

**How it works:**
- When `uniworld.enableBidiVisualCursor` is enabled, Left/Right arrow keys
  follow the visual direction rather than the logical direction.
- For each cursor movement, the extension calls `bidi_levels()` to get the
  embedding level at or near the cursor position.
- If the level is odd (RTL), the logical direction is reversed: pressing Left
  moves logically right (which is visually left in RTL text), and vice versa.
- Combined with grapheme-aware movement (`move_left`/`move_right` from WASM)
  so the cursor never lands inside a grapheme cluster.

**When clause precedence:**
- When `enableBidiVisualCursor` is enabled, it takes precedence over
  `enableGraphemeCursor` for Left/Right keys (the `when` clause for grapheme
  cursor includes `!config.uniworld.enableBidiVisualCursor`).

### 4. Line Break Opportunity Decorations (Phase 3)

**How it works:**
- When `uniworld.showLineBreakOpportunities` is enabled (or toggled via
  command palette), subtle decorations appear at every UAX #14 line break
  opportunity in the editor.
- Allowed breaks: grey middle dot (`\u00B7`).
- Mandatory breaks: small red circle (`\u25CF`).
- Only visible lines are processed for performance; decorations refresh on
  scroll, edit, and editor change.
- Uses `line_break_opportunities_with_dictionary()` from WASM, which includes
  dictionary-based word segmentation for Thai, Lao, Khmer, and Myanmar.

### 5. Grapheme-aware Word Selection (Phase 3)

**How it works:**
- `UniWorld: Select Word at Cursor` command uses `word_boundaries()` from WASM.
- Finds the word segment containing the cursor's byte position.
- Converts byte offsets back to UTF-16 char indices for VS Code positions.
- If the cursor is on whitespace, selects the next word instead.
- Respects script-specific word boundaries, unlike VS Code's default which
  uses regex-based word detection.

### 6. Package.json Updates

- Added commands: `cursorLeftVisual`, `cursorRightVisual`, `selectWord`,
  `toggleLineBreakDecorations`.
- Added settings: `enableBidiVisualCursor`, `enableGraphemeWordSelect`,
  `showLineBreakOpportunities`.
- Updated keybindings with proper `when` clause precedence.

## Phase 3 Status

| Feature | Status |
|---------|--------|
| Visual bidi cursor | Done (indexed stops + column-based line crossing) |
| Line break decorations | Done |
| Word selection | Done |
| Bidi visualization panel | Remaining |

## Technical Notes

### Build Process (wasm-pack vs manual)

`wasm-pack` had intermittent "Access is denied" errors calling `cargo metadata`
on this machine (Windows 11, sandboxed environment). Workaround: build WASM
directly with cargo and generate JS glue with `wasm-bindgen` CLI:

```powershell
$env:Path = "C:\Users\seanm\.cargo\bin;C:\Users\seanm\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin;" + $env:Path
cargo build --target wasm32-unknown-unknown --release --features wasm
wasm-bindgen target/wasm32-unknown-unknown/release/uniworld.wasm --out-dir extensions/vscode/wasm --target nodejs
```

### UTF-16/UTF-8/CodePoint Index Conversions

The bidi functions operate on code point indices (Rust `char`), while VS Code
uses UTF-16 character indices, and grapheme/linebreak functions use UTF-8 byte
offsets. The extension handles all three coordinate systems:
- UTF-16 char index <-> code point index (for bidi levels)
- UTF-16 char index <-> UTF-8 byte offset (for grapheme/word/linebreak boundaries)

### Visual cursor stops and line-crossing (v0.0.7+)

- Introduced `visual_cursor_stops(text)` in Rust, which returns a **deduplicated**
  list of UTF-8 byte offsets ordered from screen-left to screen-right. Each stop
  represents one visible cursor position; duplicates at bidi entry/exit boundaries
  are removed so arrow keys never \"bounce\" back to previously visited bytes.
- In the VS Code extension, `moveCursorVisual` now tracks a **pure index** into
  this stop list. Right = index + 1, Left = index - 1; every keypress moves one
  visual position with no need for complex disambiguation hints.
- Line-crossing is handled explicitly:
  - Right at the last stop moves to the visual left edge of the next line.
  - Left at the first stop moves to the visual right edge of the previous line.
- A small cache (`pendingCrossDirection` + target line) ensures that when the
  user lands on a new line via Up/Down or mouse, the next Left/Right press seeds
  the stop index at the correct edge, even in pure RTL paragraphs.

## Next Steps

- Implement bidi visualization panel (remaining Phase 3 item).
- Begin Phase 4: Settings UI polish, icon/branding, packaging, publishing.
