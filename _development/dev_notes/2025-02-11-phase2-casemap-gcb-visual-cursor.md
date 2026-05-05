# Phase 2 Completion - Casemap, GCB Upgrade, Visual Cursor

**Date**: 2025-02-11 (Opus session)  
**Scope**: Final Phase 2 composite operations and data quality fixes  

## Summary

This session completed all Phase 2 Rust-side composite operations and resolved a significant data quality issue in the grapheme break property tables. All 118 tests pass (including 6 Unicode conformance test suites with ~884K test cases).

## Changes

### 1. Linker Fix
- PDB lock error (LNK1201) resolved via `cargo clean`. Environmental issue, not code.

### 2. Case Mapping Module (`src/casemap/mod.rs`)

Full Unicode case mapping implementation:

- **Data generation**: `generate_casemap_tables.py` parses UnicodeData.txt (fields 12-14), SpecialCasing.txt (103 unconditional + 16 conditional entries), and CaseFolding.txt (1,512 simple + 104 full multi-char + 2 Turkic overrides).
- **Generated tables** (`src/data/casemap.rs`, 115 KB):
  - SIMPLE_LOWERCASE (1,488 entries), SIMPLE_UPPERCASE (1,505), SIMPLE_TITLECASE (58 where != upper)
  - FULL_UPPERCASE/TITLECASE/LOWERCASE (multi-char from SpecialCasing unconditional)
  - SIMPLE_CASE_FOLD (1,512), FULL_CASE_FOLD (104 multi-char)
  - TURKIC_CASE_FOLD (2), SOFT_DOTTED (18)
  - Binary search lookup functions for all tables
- **API**:
  - `to_lowercase(s)` / `to_uppercase(s)` / `to_titlecase(s)` -- full Unicode default mappings
  - `to_lowercase_locale(s, locale)` / `to_uppercase_locale(s, locale)` -- Turkish/Azerbaijani (tr, az)
  - `case_fold(s)` / `case_fold_simple(s)` / `case_fold_locale(s, locale)` -- for case-insensitive matching
  - `is_uppercase(ch)` / `is_lowercase(ch)` -- classification
- **Special rules implemented**:
  - German sharp s (U+00DF) -> "SS" in uppercase
  - Ligature expansion (ff, fi, fl, ffi, ffl, st -> expanded forms)
  - Greek final sigma: U+03A3 -> U+03C2 at word-final position (context-aware with is_cased/is_case_ignorable helpers)
  - Turkish/Azerbaijani: I -> dotless i (U+0131), i -> I with dot above (U+0130)
  - Full case folding: sharp s -> "ss" for comparison
- **18 unit tests** covering all features

### 3. GCB Data Quality Upgrade

Replaced the entire hand-crafted `src/data/grapheme_break.rs` with auto-generated tables from authoritative Unicode data:

- **Scripts created**:
  - `generate_gcb_tables.py` -- parses 3 UCD files and generates Rust source
- **Data sources**:
  - `GraphemeBreakProperty.txt` -- 13 GCB classes, 18,100 code points
  - `DerivedCoreProperties.txt` -- InCB_Consonant (911 cps), InCB_Linker (20 cps)
  - `emoji-data.txt` -- Extended_Pictographic (2,848 cps in 156 ranges)
- **Key fixes**:
  - SpacingMark: 5 entries -> 381 code points (158 ranges). Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam, and dozens more Indic + Southeast Asian scripts now correctly clustered.
  - Extended_Pictographic: approximate ranges -> exact ranges from emoji-data.txt. Fixed false positives (e.g. U+2701 UPPER BLADE SCISSORS was incorrectly classified as ExtPict).
  - InCB_Extend handled correctly: NOT mapped to GCB (separate property), avoiding range table overlap bugs.
- **Conformance**: 766/766 GraphemeBreakTest.txt cases pass (was 761/766 during intermediate fix).

### 4. Visual-Order Cursor Navigation

Added bidi-aware visual cursor movement to `src/cursor/mod.rs`:

- `move_right_visual(text, current)` -- move one grapheme cluster screen-rightward
- `move_left_visual(text, current)` -- move one grapheme cluster screen-leftward
- Uses UAX #9 bidi resolution + grapheme cluster boundaries
- Internal `build_clusters` maps grapheme clusters to bidi embedding levels
- Internal `visual_order` implements L2 reversal on cluster level
- 4 new unit tests (pure LTR, pure RTL, empty, boundaries)

### 5. Integration Tests (`tests/composite_operations.rs`)

34 new integration tests across all composite modules:

- **Cursor** (11): Devanagari clusters, Thai text, emoji flags, skin tones, mixed-script word selection, multibyte delete, visual cursor
- **Width** (8): CJK ideographs, Hangul, Katakana, combining sequences, ZWJ, mixed script, controls, diacritics
- **Truncate** (7): Emoji sequences, Devanagari, CJK boundaries, mixed ASCII+CJK, regional indicators
- **Casemap** (10): Latin extended, Greek, Cyrillic, Turkish roundtrip, titlecase, case folding, locale, classification, CJK no-case

## Test Results

```
118 tests total:
  46 unit tests (lib) .............. ok
  34 composite operations .......... ok
   2 bidi conformance (861,948) .... ok
   1 grapheme conformance (766) .... ok
   1 linebreak conformance (19,338)  ok
   1 normalization conformance ..... ok
   1 sentence conformance (512) .... ok
   1 word conformance (1,944) ...... ok
  15 dictionary segmentation ....... ok
  16 segment tests ................. ok
```

## Files Modified/Created

- `src/casemap/mod.rs` -- full implementation (was stub)
- `src/data/casemap.rs` -- auto-generated case mapping tables (NEW, 115 KB)
- `src/data/grapheme_break.rs` -- regenerated from authoritative data (was hand-crafted)
- `src/data/mod.rs` -- added `pub mod casemap`
- `src/cursor/mod.rs` -- added visual-order navigation functions
- `_development/scripts/generate_casemap_tables.py` -- NEW
- `_development/scripts/generate_gcb_tables.py` -- NEW
- `tests/composite_operations.rs` -- NEW (34 integration tests)

## Phase 2 Composite Operations Status

All Rust-side composite operations are complete:
- [x] cursor (logical + visual-order)
- [x] width (display width)
- [x] truncate (safe truncation)
- [x] casemap (locale-aware case mapping)

Remaining Phase 2 work: language bindings (Python/PyO3, JS/WASM, C, Go).
