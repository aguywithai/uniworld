# Dev notes -- 2025-02-11: UAX #9 Bidirectional Algorithm -- COMPLETE

## Results

- **BidiTest.txt**: 770,241/770,241 passed (class-level, all paragraph directions)
- **BidiCharacterTest.txt**: 91,707/91,707 passed (character-level, bracket pairing)
- No regressions in any other conformance suite.

## Files created/modified

### New files
- `_development/scripts/generate_bidi_tables.py` -- generates Rust data tables from
  UnicodeData.txt (Bidi_Class field 4) and BidiBrackets.txt (bracket pairs).
- `src/data/bidi_class.rs` (auto-generated) -- BidiClass enum (23 types), range table
  (810 entries), bidi_class() lookup, bracket pair/closing bracket lookups.
- `src/bidi/mod.rs` -- full UAX #9 implementation: resolve() for strings,
  resolve_classes() for class sequences.
- `tests/conformance_bidi.rs` -- conformance tests against both test files.

### Modified files
- `_development/scripts/download_ucd_tests.ps1` -- added BidiTest.txt,
  BidiCharacterTest.txt, BidiBrackets.txt downloads.
- `src/data/mod.rs` -- added `pub mod bidi_class`.
- `.cursor/rules/PROJECT_ROADMAP.mdc` -- marked UAX #9 complete.

## Algorithm architecture

The implementation follows UAX #9 strictly:

1. **P2/P3**: Paragraph level detection with isolate-aware scanning.
2. **X1-X8**: Stack-based explicit embedding/override/isolate processing.
   - Directional status stack with level, override, isolate fields.
   - Overflow counters for isolate and embedding depth.
   - FSI resolved via P2/P3 on isolate content.
3. **X9**: Remove embedding/override controls and BN (using original classes
   to detect BN that may have been overridden by X6).
4. **X10**: Isolating run sequence construction.
   - Level runs identified from non-removed characters.
   - Isolate initiator/PDI matching from original classes.
   - Run chaining: initiator's run chains to PDI's run.
   - Proper sos/eos computation including unmatched isolate initiator case
     (use para_level, not content-inside-isolate level).
5. **W1-W7**: Weak type resolution (NSM, EN/AN, AL->R, ES/CS/ET, W7 L context).
6. **N0**: Bracket pair resolution per BD16.
   - Stack limit 63 with "stop processing" on overflow.
   - Canonical equivalents for bracket matching (U+2329<->U+3008 etc.).
   - Only characters with current bidi class ON considered for bracket pairing.
   - Pre-W1 NSM positions tracked; NSMs following resolved brackets get
     bracket type.
   - Embedding direction check: matching types first (b), then opposite with
     context search (c).
7. **N1-N2**: Neutral/isolate formatting type resolution with proper
   embedding direction from sequence level.
8. **I1-I2**: Implicit level adjustment.
9. **L1**: Reset levels of WS/isolate formatting at line boundaries;
   restore X9-removed characters to u8::MAX after L1.
10. **L2**: Visual reorder by reversing segments at each level from
    highest down to lowest odd.

## Key fixes during development

1. **X9 original classes** (304K -> 18K failures): X9 must check original_classes
   (before X6 overrides), not the modified classes, to correctly remove BN chars
   whose class was overridden to L/R.

2. **Unmatched isolate initiator eos** (18K -> 4.4K failures): When a sequence
   ends with an isolate initiator without matching PDI, eos uses para_level
   (not the level of content inside the isolate scope).

3. **L1 vs X9 restoration** (remaining from fix 1): L1 was overwriting X9-removed
   characters' u8::MAX level with para_level. Added post-L1 restoration step.

4. **BD16 ON-only bracket identification** (30 -> 5 failures): Characters whose
   bidi class was changed by overrides (e.g. `)` overridden to L by LRO) must
   not be considered as brackets per spec.

5. **BD16 stack overflow = stop** (5 -> 0 failures): When the BD16 bracket
   stack is full (63 entries), the spec says "stop processing" -- abort the
   entire bracket identification, not just skip the current bracket.

6. **Canonical bracket equivalents** (5 -> 0 failures): U+2329/U+232A decompose
   to U+3008/U+3009; bracket pairing must use canonical equivalents.

7. **Pre-W1 NSM tracking for N0** (30 -> 5 failures): After W1 changes NSM
   to the preceding type, N0 can't detect them. Track original NSM positions
   before W rules and apply bracket type to them after N0 resolves.

## Complexity notes

UAX #9 is the most complex Unicode algorithm. Key sources of complexity:
- The stack-based X rules with three kinds of scoping (embedding, override, isolate)
  and overflow counters for each.
- Isolating run sequences requiring matching of initiators to PDIs across
  potentially many level runs.
- N0 bracket pairing with its own stack, canonical equivalents, override-awareness,
  NSM handling, and the embedding-vs-context direction decision tree.
- The interaction between L1 and X9 (L1 must not overwrite X9 removals).

Despite this complexity, the implementation achieved full conformance in a single
development session with iterative fixes that followed directly from analyzing
test failure patterns.
