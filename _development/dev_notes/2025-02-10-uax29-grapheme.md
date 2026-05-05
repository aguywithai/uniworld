# Dev notes — 2025-02-10: UAX #29 grapheme cluster implementation

## Cargo

Cargo was not in PATH in the automation environment; Rust may not be installed there. Run locally: `cargo build` and `cargo test` to verify.

## Done this session

- **data/grapheme_break.rs**
  - `Gcb` enum (Cr, Lf, Control, Extend, Zwj, RegionalIndicator, Prepend, SpacingMark, L, V, T, Lv, Lvt, Other).
  - `gcb(char)` with minimal inline table: CR/LF/ZWJ/ZWNJ, RI range, Hangul syllable/jamo, Control (C0/C1), Extend (Mn/Me and many ranges from UCD), SpacingMark (Thai 0E33, Lao 0EB3). PUA and other unassigned left as Other.
  - Full Extend coverage is a large set; added a representative set of combining mark ranges; ucd_gen can replace with GraphemeBreakProperty.txt later for conformance.

- **segment/grapheme.rs**
  - UAX #29 extended grapheme boundary rules: GB3 (CR x LF), GB4 (Control/CR/LF break), GB5–GB7 (Hangul L/V/T/LV/LVT), GB8 (RI x RI with even count), GB9 (x Extend), GB9a (x SpacingMark).
  - `grapheme_boundaries()` and `GraphemeClusterBoundaries` iterator now use GCB and break rules.
  - GB11 (ZWJ x Extended_Pictographic) and Prepend (GB9b) not yet implemented; can add when we have Extended_Pictographic and Prepend data.

- **tests/segment_tests.rs**
  - Tests: empty, ASCII, iterator; base+Extend (e+acute = one cluster); CRLF = one cluster; two RIs = one cluster; four RIs = two clusters.

## Next steps (historical)

- Conformance test added; GB11 and ConjunctLinker implemented. Grapheme: **766/766 pass.** See `2025-02-10-grapheme-complete-phase1-status.md` for current status and roadmap.
