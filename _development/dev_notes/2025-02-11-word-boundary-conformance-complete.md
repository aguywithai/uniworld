# Dev notes -- 2025-02-11: Word boundary conformance complete (1944/1944)

## Summary

Word boundary segmentation (UAX #29) now passes 100% of the official Unicode 17.0 WordBreakTest.txt conformance suite: **1944/1944 test cases pass**. Combined with grapheme (766/766), the text segmentation work for grapheme and word is conformance-grade. Sentence boundaries remain stubbed.

## What was done

### Complete rewrite of word boundary logic (src/segment/word.rs)

The previous incremental approach was patching individual failures without getting the architecture right. A clean rewrite based on the actual UAX #29 rule ordering resolved the bulk of failures in one pass.

**Architecture:**
- Pre-collect all characters, their WB properties, and byte offsets into vectors.
- For each adjacent pair, apply rules in the correct UAX #29 order:
  1. WB3-WB3c on RAW properties (CR x LF, Newline breaks, ZWJ x Extended_Pictographic).
  2. WB3d: WSegSpace x WSegSpace (horizontal whitespace clustering).
  3. WB4: Extend/Format/ZWJ transparency -- do not break before transparent chars.
  4. WB5-WB999 on RESOLVED properties (skip Extend/Format/ZWJ to find base characters).
- Helper functions `resolve_prev`, `resolve_prev_prev`, `resolve_next_next` skip transparent characters to find the base WB property for each position.
- RI (Regional_Indicator) pairing tracked with a running counter for WB15/WB16.

**Key insight:** WB3c (ZWJ x Extended_Pictographic) applies only on RAW adjacent characters, not through transparent characters. ZWJ is itself transparent for WB4, so after WB3c, ZWJ is "invisible" for all remaining rules. The previous code incorrectly treated ZWJ as a word-connector.

### Data classification fixes (src/data/word_break.rs)

**Property reclassifications:**
- U+003A (COLON): MidNumLet -> **MidLetter** (affects WB6/WB7 vs WB11/WB12)
- U+002E (FULL STOP): MidNum -> **MidNumLet** (participates in both letter and numeric mid-word rules)
- U+0387, U+055F, U+FE13, U+FE55, U+FF1A: moved from MidNumLet to **MidLetter**
- Added U+FE52, U+FF0E, U+FF07 to **MidNumLet**
- Removed U+FE52, U+FF0E from MidNum

**Unicode 17.0 reclassifications:**
- U+06DD (ARABIC END OF AYAH): was Format, now **Numeric**
- U+070F (SYRIAC ABBREVIATION MARK): was Format, now **ALetter**

**New: `is_extended_pictographic(c: char)` function:**
- Covers emoji-data.txt Extended_Pictographic ranges for WB3c.
- Excludes U+2701 (UPPER BLADE SCISSORS) which Unicode 17.0 removed from Extended_Pictographic.
- Range 0x2600-0x27BF split to 0x2600-0x2700 | 0x2702-0x27BF.

**Extend additions:**
- U+1F3FB-1F3FF (emoji skin tone modifiers): GC=Sk but Word_Break=Extend.
- U+E0100-E01EF (Variation Selectors Supplement).

**Other cleanup:**
- Removed dead code: U+3099-309A in Katakana check (already caught by is_extend).
- Removed U+06DD and U+070F from is_format.

### Rules implemented (complete UAX #29 WB rule set)

- WB3: CR x LF
- WB3a/3b: Newline/CR/LF breaks
- WB3c: ZWJ x Extended_Pictographic
- WB3d: WSegSpace x WSegSpace
- WB4: Extend/Format/ZWJ transparency
- WB5: AHLetter x AHLetter
- WB6/WB7: AHLetter x (MidLetter|MidNumLetQ) AHLetter (with lookahead/lookback)
- WB7a: Hebrew_Letter x Single_Quote
- WB7b/WB7c: Hebrew_Letter x Double_Quote Hebrew_Letter (with lookahead/lookback)
- WB8: Numeric x Numeric
- WB9: AHLetter x Numeric
- WB10: Numeric x AHLetter
- WB11/WB12: Numeric (MidNum|MidNumLetQ) x Numeric (with lookahead/lookback)
- WB13: Katakana x Katakana
- WB13a: (AHLetter|Numeric|Katakana|ExtendNumLet) x ExtendNumLet
- WB13b: ExtendNumLet x (AHLetter|Numeric|Katakana)
- WB15/WB16: Regional_Indicator pairing
- WB999: Any / Any (default break)

## Files changed

- `src/segment/word.rs` -- complete rewrite
- `src/data/word_break.rs` -- classification fixes, is_extended_pictographic, is_extend additions

## Test results

- Grapheme conformance: 766/766 pass
- Word conformance: 1944/1944 pass (Unicode 17.0 WordBreakTest.txt)
- Unit tests: 10/10 pass

## Commands

- Full suite: `cargo test`
- Word only: `cargo test --test conformance_word`
- Grapheme only: `cargo test --test conformance_grapheme`

## What remains for Phase 1 Week 1-2

- **Sentence boundaries:** Still stubbed. SentenceBreakTest.txt is downloaded. Need Sentence_Break property, default rules, and conformance test.

## Lessons learned

1. **Get the architecture right first.** The incremental patching approach (fix one test, break another) went through many iterations. The clean rewrite based on the spec's rule ordering resolved ~100 failures at once.
2. **Property classifications matter.** MidLetter vs MidNumLet vs MidNum distinctions are subtle but cause cascading failures when wrong.
3. **Unicode version matters.** The test file is Unicode 17.0. Properties like U+06DD, U+070F, and U+2701 changed between versions. Always check the test file version.
4. **WB4 transparency is the key insight.** All rules after WB3d operate on "resolved" properties (skipping Extend/Format/ZWJ). Getting this right eliminates a huge class of bugs.
