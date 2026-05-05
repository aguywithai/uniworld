# Dev notes -- 2025-02-11: Starting UAX #14 Line Breaking

## Context

Phase 1 progress: 3/4 complete. This is the last core algorithm.
- [x] UAX #29 Text Segmentation (grapheme/word/sentence)
- [x] UAX #15 Normalization (NFC/NFD/NFKC/NFKD)
- [x] UAX #9 Bidirectional Algorithm
- [ ] UAX #14 Line Breaking (this session)

## Scope

UAX #14 defines where line breaks may occur in text. The algorithm has:
- ~43 Line_Break property classes (AL, BA, BB, BK, CJ, CL, CM, CP, CR, EX,
  GL, H2, H3, HL, HY, ID, IN, IS, JL, JT, JV, LF, NL, NS, NU, OP, PO, PR,
  QU, RI, SA, SG, SP, SY, WJ, XX, ZW, ZWJ, AI, CB, EB, EM, plus others)
- Rules LB1-LB30+ (mandatory breaks, combining marks, non-breaking, spaces,
  break opportunities, CJK, numeric, Korean Jamo, etc.)

## Data needs

1. `LineBreak.txt` -- Line_Break property values for all code points (ranges)
2. `LineBreakTest.txt` -- conformance test data (similar format to other break tests)

## Approach

1. Download LineBreak.txt and LineBreakTest.txt
2. Generate line break property table (Python script -> src/data/line_break.rs)
3. Implement UAX #14 algorithm in src/linebreak/mod.rs
4. Create conformance tests
5. Dictionary integration for Thai/Lao/Khmer/Myanmar deferred to Phase 2
   (not required for the rule-based conformance tests)

## Key challenges

- The pair table approach: many rules depend on the combination of the
  break class before and after the break point.
- Combining marks (CM/ZWJ) transparency handling similar to word/sentence break.
- Regional indicator pairing (similar to WB15/16).
- Korean Jamo and Hangul syllable type interactions.
- Numeric context rules (LB25) are complex.
