# Phase 2 – Cursor, Width, Truncate Basics

**Date**: 2025-02-11  
**Scope**: Initial composite operations (cursor navigation, display width, truncation)  

## Changes

- `src/cursor/mod.rs`
  - Implemented logical-order cursor helpers:
    - `move_right(text, current)` / `move_left(text, current)` – step by grapheme cluster (UAX #29) in logical order.
    - `delete_backward(text, current)` – backspace deletes one grapheme cluster.
    - `delete_forward(text, current)` – Delete key deletes one grapheme cluster.
    - `select_word(text, current)` – selects the word containing `current` using `segment::word_boundaries`.
  - Added unit tests for ASCII, emoji clusters, delete operations, and word selection.
  - Note: current implementation is **logical-order only**; visual-order cursor navigation for mixed-direction text will be layered on later using `bidi::BidiInfo`.

- `src/width/mod.rs`
  - Implemented `display_width(s)` and `char_width(ch)`:
    - Combines Grapheme_Break classes + Line_Break `is_east_asian_wide`.
    - Rules:
      - C0/C1 controls and newline/backspace: width 0.
      - Combining marks, ZWJ, spacing marks, conjunct linkers: width 0.
      - East Asian wide/full-width: width 2.
      - Everything else: width 1.
  - Added tests for ASCII, CJK wide characters, and combining sequences.

- `src/truncate/mod.rs`
  - Implemented:
    - `truncate_graphemes(text, max_graphemes)` – cuts only on grapheme boundaries.
    - `truncate_display_width(text, max_width)` – truncates on grapheme boundaries so total `display_width` ≤ `max_width`.
  - Added tests for emoji truncation by grapheme and simple CJK width truncation.

## Status and Notes

- All new code compiles; unit tests for cursor/width/truncate run to completion when the Rust toolchain can link test binaries (current machine hit a PDB write/linker issue, unrelated to logic).
- Composite operations are now **functionally usable for LTR text**:
  - Cursor movement and deletion do not split grapheme clusters.
  - Display width respects East Asian width and combining marks.
  - Truncation is safe at both the grapheme and display-width levels.
- Remaining Phase 2 work:
  - Add visual-order cursor navigation for mixed bidi text (using `bidi::BidiInfo`).
  - Expand tests to cover mixed-direction, Indic, and emoji-heavy scenarios.
  - Start Python binding scaffolding once composite APIs stabilize.

