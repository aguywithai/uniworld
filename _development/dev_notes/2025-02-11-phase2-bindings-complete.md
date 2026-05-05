# Phase 2 Bindings - Python, WASM, C, Go

**Date**: 2025-02-11 (Opus session, continued)  
**Scope**: Language bindings for all four target platforms  

## Summary

All four binding targets are scaffolded and compile-verified. Each is behind a feature flag to keep the default library lightweight.

## Python Bindings (PyO3)

**Feature**: `--features python`  
**Build**: `maturin build --features python`  

**File**: `src/python_bindings.rs` (24 exposed functions)  
**Config**: `pyproject.toml` (maturin build system)

Exposes the full API surface:
- Segmentation: `grapheme_boundaries`, `word_boundaries`, `sentence_boundaries`
- Normalization: `normalize_nfc`, `normalize_nfd`, `normalize_nfkc`, `normalize_nfkd`
- Bidi: `bidi_resolve` (returns dict with paragraph_level, levels, reorder)
- Line breaking: `line_break_opportunities`, `line_break_opportunities_with_dictionary` (returns (offset, action) tuples)
- Case mapping: `to_lowercase`, `to_uppercase`, `to_titlecase`, `case_fold`, `case_fold_simple` (with locale support)
- Width: `display_width`
- Truncation: `truncate_graphemes`, `truncate_display_width`
- Cursor: `move_right`, `move_left`, `move_right_visual`, `move_left_visual`, `select_word`

## JavaScript/WASM Bindings (wasm-bindgen)

**Feature**: `--features wasm`  
**Build**: `wasm-pack build --features wasm`  

**File**: `src/wasm_bindings.rs`

Covers core operations suitable for browser/Node.js use:
- Segmentation (grapheme, word, sentence)
- Normalization (NFC, NFD, NFKC, NFKD)
- Case mapping (lowercase, uppercase, titlecase, case_fold)
- Width and truncation
- Cursor movement (logical order)

## C FFI Bindings

**Feature**: `--features cffi`  
**Build**: `cargo build --release --features cffi`  

**File**: `src/c_bindings.rs`

Provides `extern "C"` functions with `#[no_mangle]`:
- `uniworld_normalize_nfc`, `uniworld_normalize_nfd`
- `uniworld_to_lowercase`, `uniworld_to_uppercase`
- `uniworld_display_width`
- `uniworld_move_right`, `uniworld_move_left`
- `uniworld_free_string` (memory management for returned strings)

All string functions accept null-terminated UTF-8 C strings and return owned C strings that must be freed by the caller.

## Go Bindings (CGo)

**File**: `bindings/go/uniworld.go`

Wraps the C FFI with idiomatic Go:
- `NormalizeNFC(text string) string`
- `NormalizeNFD(text string) string`
- `ToLowercase(text string) string`
- `ToUppercase(text string) string`
- `DisplayWidth(text string) uint32`
- `MoveRight(text string, current uint64) uint64`
- `MoveLeft(text string, current uint64) uint64`

Proper memory management: C strings allocated/freed on each call.

## Compilation Status

All feature flags compile cleanly:
- `cargo check` (default) -- ok
- `cargo check --features python` -- ok (PyO3 0.23.5)
- `cargo check --features cffi` -- ok
- `cargo check --features wasm` -- ok (wasm-bindgen 0.2.108)
- `cargo test` (default) -- 118/118 tests pass

## Phase 2 Status

**Phase 2 is now complete:**
- [x] Composite operations: cursor, width, truncate, casemap
- [x] GCB data quality upgrade (authoritative tables)
- [x] Bindings: Python, WASM, C, Go

Next: Phase 3 (Testing and Documentation) and Phase 4 (Community and Ecosystem).
