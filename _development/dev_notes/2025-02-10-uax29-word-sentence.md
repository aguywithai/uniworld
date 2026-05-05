# Dev notes — 2025-02-10: UAX #29 word and sentence boundaries

## Goal

Complete Phase 1 Week 1-2 (Text Segmentation) by implementing UAX #29 word and sentence boundaries. Grapheme is done (766/766). Word and sentence were stubbed.

## Done this session (word boundaries)

1. **Word_Break property** — `src/data/word_break.rs`
   - `Wb` enum: Cr, Lf, Newline, Extend, Format, Zwj, ALetter, HebrewLetter, Numeric, Katakana, SingleQuote, DoubleQuote, MidLetter, MidNumLet, MidNum, WSegSpace, Other.
   - `wb(c)` with inline classification: CR/LF/Newline, ZWJ/ZWNJ, quotes, Format, Extend (large combining set), WSegSpace, Hebrew_Letter, Katakana, Mid*, Numeric, ALetter (Lu/Ll/Lt/Lm/Lo ranges), default Other.
2. **Word boundary rules** — `src/segment/word.rs`
   - `word_boundaries(s, locale)` returns byte offsets of word starts; locale reserved.
   - `break_between(prev_prev, prev, next)` implements WB3–WB15 class: CR×LF, (Newline|CR|LF)÷, ÷(Newline|CR|LF), ×Extend, ×Format, ×ZWJ, (ALetter|Hebrew_Letter)×(ALetter|Hebrew_Letter), Mid* and Numeric rules, Katakana×Katakana, quote handling.
3. **Download script** — WordBreakTest.txt and SentenceBreakTest.txt added to `download_ucd_tests.ps1`.
4. **Conformance** — `tests/conformance_word.rs` parses WordBreakTest.txt (÷/×) and compares to `word_boundaries()`; skips if file missing.
5. **Unit tests** — `word_boundaries_empty`, `word_boundaries_hello_world`, `word_boundaries_single_word` in segment_tests.rs.

## Plan (sentence boundaries)

- Add `Sentence_Break` property and default rules; add conformance test for SentenceBreakTest.txt.

## References

- UAX #29: https://www.unicode.org/reports/tr29/
- Word_Break: UCD auxiliary WordBreakProperty.txt
- Sentence_Break: UCD auxiliary (derived or in UAX #29)
- Tests: WordBreakTest.txt, SentenceBreakTest.txt in UCD auxiliary

## Implementation approach

- Word/sentence boundaries are defined at code-point offsets; we return byte offsets. So we iterate by character (code point), compute break between prev and next, and emit byte offset at each break (start of each word/sentence).
- Minimal WB property: CR, LF, Newline, Extend, Format, ZWJ, ALetter (from GC), Hebrew_Letter (range), Numeric (Nd), Katakana (range), Single_Quote, Double_Quote, MidNumLet, MidLetter, MidNum, WSegSpace (or use Other for space). Default = Other.
- Rules: WB1 sot ÷, WB2 ÷ eot, WB3 CR × LF, WB4/WB5 at newlines, WB6 × Extend, WB7 × Format, WB7a × ZWJ (emojis), then letter/num/mid rules.
