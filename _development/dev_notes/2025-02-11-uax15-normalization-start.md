# Dev notes -- 2025-02-11: Starting UAX #15 Normalization

## Goal

Implement Unicode Normalization Forms (NFC, NFD, NFKC, NFKD) per UAX #15, pass NormalizationTest.txt conformance suite.

## Context

Phase 1 Week 1-2 (Text Segmentation) is complete. All three UAX #29 algorithms pass conformance:
- Grapheme: 766/766
- Word: 1944/1944
- Sentence: 512/512

Now starting Week 2-3: Normalization.

## Plan

1. Download additional UCD files: UnicodeData.txt, CompositionExclusions.txt, NormalizationTest.txt
2. Create Python script to parse UnicodeData.txt and generate Rust normalization tables:
   - Canonical Combining Class (CCC) lookup
   - Canonical decomposition mappings
   - Compatibility decomposition mappings
   - Canonical composition table (derived from decompositions minus exclusions)
3. Implement normalization in src/normalize/mod.rs:
   - NFD: recursive canonical decomposition + CCC sort
   - NFC: NFD + canonical composition
   - NFKD: recursive compatibility decomposition + CCC sort
   - NFKC: NFKD + canonical composition
4. Conformance tests against NormalizationTest.txt

## Key data needs

- UnicodeData.txt: ~34,000 lines, contains CCC and decomposition mappings
- CompositionExclusions.txt: ~50 entries of characters excluded from composition
- NormalizationTest.txt: ~18,000+ test cases

## Approach notes

Unlike UAX #29 (rule-driven), UAX #15 is data-driven. The algorithm is straightforward:
- Decompose: replace each character with its decomposition mapping (recursively)
- Sort: reorder combining marks by CCC (stable sort)
- Compose (for NFC/NFKC): combine adjacent composable pairs

The challenge is getting the data tables right and handling all edge cases in the composition algorithm (starter + non-starter composition, blocked pairs, etc.).
