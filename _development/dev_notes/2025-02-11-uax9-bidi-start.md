# Dev notes -- 2025-02-11: Starting UAX #9 Bidirectional Algorithm

## Context

Phase 1 is now 50% complete:
- [x] UAX #29 Text Segmentation (grapheme 766/766, word 1944/1944, sentence 512/512)
- [x] UAX #15 Normalization (NFC/NFD/NFKC/NFKD, all NormalizationTest.txt passing)
- [ ] UAX #9 Bidirectional Algorithm (this session)
- [ ] UAX #14 Line Breaking

## Scope

UAX #9 is the most complex Unicode algorithm. It resolves visual ordering of mixed
left-to-right and right-to-left text. The algorithm has these phases:

1. **P1-P3**: Paragraph-level direction detection
2. **X1-X10**: Explicit directional embeddings and overrides (embedding levels)
3. **W1-W7**: Weak type resolution
4. **N0-N2**: Neutral type resolution (including bracket pairing BD16)
5. **I1-I2**: Implicit level resolution
6. **L1-L4**: Reordering resolved levels to visual order

## Data needs

- Bidi_Class property for all code points (23 types: L, R, AL, EN, ES, ET, AN, CS,
  NSM, BN, B, S, WS, ON, LRE, RLE, LRO, RLO, PDF, LRI, RLI, FSI, PDI)
- BidiTest.txt (~500,000 test cases) and BidiCharacterTest.txt
- Bracket pairs for BD16 (BidiBrackets.txt)

## Approach

1. Generate bidi class data from UnicodeData.txt (field 4 is Bidi_Class)
2. Download BidiTest.txt, BidiCharacterTest.txt, BidiBrackets.txt
3. Implement algorithm phases in order
4. Start with BidiCharacterTest.txt (simpler format, character-level)
5. Then tackle BidiTest.txt (exhaustive, class-level)

## Key challenges

- The X1-X10 explicit embedding rules use a stack-based state machine with
  max depth 125 and overflow handling
- Bracket pairing (N0/BD16) requires matching brackets and assigning them
  the direction of the enclosing context
- The test suite is massive (~500,000 cases)

## Plan

Given complexity, the implementation will be split:
1. Bidi_Class property table (data/bidi_class.rs)
2. Core algorithm struct and phases (bidi/mod.rs or bidi/resolve.rs)
3. Bracket pair data and matching (data/bidi_brackets.rs or within bidi module)
4. Conformance tests (tests/conformance_bidi.rs)
