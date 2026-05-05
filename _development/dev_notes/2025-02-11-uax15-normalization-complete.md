# Dev notes -- 2025-02-11: UAX #15 Normalization complete

## Summary

Unicode Normalization (UAX #15) is now implemented and passes the full NormalizationTest.txt conformance suite (Unicode 17.0). All four normalization forms (NFC, NFD, NFKC, NFKD) work correctly.

Combined with the UAX #29 segmentation work, we now have four conformance-grade Unicode algorithms:
- Grapheme cluster boundaries: 766/766
- Word boundaries: 1944/1944
- Sentence boundaries: 512/512
- Normalization (NFC/NFD/NFKC/NFKD): ~20,000 test cases, all passing

## What was done

### Data generation pipeline

Created `_development/scripts/generate_normalization_tables.py` which:
1. Downloads UnicodeData.txt and CompositionExclusions.txt via updated download script
2. Parses CCC (Canonical Combining Class) for 968 characters
3. Extracts 2081 canonical decomposition mappings
4. Extracts 5914 total compatibility decomposition mappings (3833 compatibility-only)
5. Derives 961 canonical composition pairs (filtering exclusions)
6. Generates `src/data/normalization.rs` (253.8 KB) with binary-search lookup tables

### Algorithm implementation (src/normalize/mod.rs)

**NFD (Canonical Decomposition):**
- Recursively decompose each character using canonical decomposition table
- Hangul syllables decomposed algorithmically (LVT -> L + V + T)
- Reorder combining marks by CCC (bubble sort, stable)

**NFC (Canonical Decomposition + Canonical Composition):**
- Apply NFD first
- Scan for starters (CCC=0), attempt to compose with following characters
- Blocked-pair detection: a character is blocked from composing if an intervening
  character has CCC >= its CCC
- Hangul Jamo composition: L+V -> LV, LV+T -> LVT (algorithmic)
- Table-based composition for all other pairs

**NFKD/NFKC:** Same as NFD/NFC but using compatibility decomposition table.

### Hangul special handling

Hangul syllables (U+AC00-D7A3) use algorithmic composition/decomposition per Unicode Chapter 3:
- Constants: SBase=0xAC00, LBase=0x1100, VBase=0x1161, TBase=0x11A7
- LCount=19, VCount=21, TCount=28, NCount=588, SCount=11172
- Decomposition: syllable -> leading + vowel + optional trailing
- Composition: leading + vowel -> LV, LV + trailing -> LVT

This was the only fix needed after the initial implementation -- the first run had 11,300 failures (all Hangul-related), and adding algorithmic Hangul handling resolved all of them in one pass.

## Files created/changed

- `_development/scripts/generate_normalization_tables.py` -- new (data generator)
- `_development/scripts/download_ucd_tests.ps1` -- updated (added UCD file downloads)
- `src/data/normalization.rs` -- new (auto-generated, 253.8 KB, 8657 lines)
- `src/data/mod.rs` -- added normalization module
- `src/normalize/mod.rs` -- complete rewrite (was stub)
- `tests/conformance_normalization.rs` -- new (conformance test)

## Performance note

The conformance test takes ~8-10 seconds in debug mode due to the volume of test data (~20,000 lines x 20 checks each). This could be optimized with:
- Quick-check properties (skip normalization for already-normalized strings)
- Streaming normalization (avoid allocating full decomposition buffers)
- Release mode compilation

These are Phase 3 optimizations and don't affect correctness.

## What remains for Phase 1

- [x] Week 1-2: Text Segmentation (UAX #29) -- complete
- [x] Week 2-3: Normalization (UAX #15) -- complete
- [ ] Week 3-5: Bidirectional Algorithm (UAX #9)
- [ ] Week 5-6: Line Breaking (UAX #14)

Phase 1 is now approximately 50% complete (2 of 4 algorithm blocks done).

## Commands

- Full suite: `cargo test`
- Normalization only: `cargo test --test conformance_normalization`
- Regenerate tables: `python _development/scripts/generate_normalization_tables.py`
