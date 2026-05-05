# Dev notes -- 2025-02-11: Sentence boundary conformance complete (512/512)

## Summary

Sentence boundary segmentation (UAX #29) now passes 100% of the official Unicode 17.0 SentenceBreakTest.txt conformance suite: **512/512 test cases pass**. This completes Phase 1 Week 1-2 (Text Segmentation). All three segmentation algorithms are conformance-grade:
- Grapheme: 766/766
- Word: 1944/1944
- Sentence: 512/512

## What was done

### New file: src/data/sentence_break.rs

Sentence_Break property classifier (`Sb` enum and `sb()` function).

**Property values:**
- `Cr`, `Lf`, `Sep` -- paragraph separators
- `Extend`, `Format` -- transparent per SB5
- `Sp` -- whitespace (Zs + tab)
- `Lower`, `Upper` -- cased letters (Ll, Lu)
- `OLetter` -- other letters (Lo, Lt, Lm, CJK, Arabic, Hebrew, Thai, etc.)
- `Numeric` -- decimal digits (Nd)
- `ATerm` -- sentence-ending period (U+002E and variants)
- `STerm` -- sentence-ending terminal (!, ?, ideographic full stop, etc.)
- `Close` -- closing/quoting punctuation (Pe, Pf, some Pi)
- `SContinue` -- sentence-continuing punctuation (comma, colon, dashes)
- `Other` -- default

**Key classification details:**
- U+01BB (LATIN LETTER TWO WITH STROKE): correctly classified as OLetter (GC=Lo), not Lower. Required excluding known Lo characters from the is_lower heuristic.
- Comprehensive STerm list covering scripts from Armenian to Vai.
- Close covers Pe (closing brackets), Pf (final quotes), and some Pi (initial quotes) per Unicode behavior.
- ZWJ (U+200D) and ZWNJ (U+200C) classified as Extend per SB spec.

### Rewritten: src/segment/sentence.rs

Full UAX #29 sentence boundary algorithm (SB1-SB998).

**Architecture:**
- Pre-collect characters, SB properties, and byte offsets.
- Apply SB3-SB4 on RAW properties (CR x LF, ParaSep breaks).
- SB5: do not break before Extend/Format (transparency).
- SB6-SB998: track ATerm/STerm context using a state enum (`SbCtx`).
- SB8 lookahead implemented as a forward scan for Lower.

**State tracking (`SbCtx`):**
```
None                              -- no active sentence terminator
SATerm { is_aterm, after_close }  -- after ATerm/STerm, possibly Close*
SATermSp { is_aterm }             -- after ATerm/STerm Close* Sp+
```

**Rules implemented:**
- SB3: CR x LF (no break)
- SB4: (Sep|CR|LF) / (break after paragraph separator)
- SB5: x (Extend|Format) -- transparency
- SB6: ATerm x Numeric (decimal point, e.g. "3.4") -- only when ATerm is immediate
- SB7: (Upper|Lower) ATerm x Upper (abbreviation, e.g. "U.S.") -- only when ATerm is immediate
- SB8: ATerm Close* Sp* x [not significant]* Lower (abbreviation + lowercase continuation)
- SB8a: SATerm Close* Sp* x (SContinue|STerm|ATerm) (sentence continuation)
- SB9: SATerm Close* x (Close|Sp|ParaSep) (within sentence-end sequence)
- SB10: SATerm Close* Sp* x (Sp|ParaSep) (within sentence-end sequence)
- SB11: SATerm Close* Sp* ParaSep? / (sentence break)
- SB998: Any x Any (default no break)

### New test: tests/conformance_sentence.rs

Parses SentenceBreakTest.txt and validates all sentence boundary decisions.

### Unit tests added to tests/segment_tests.rs

6 new tests: empty, single sentence, period+uppercase, abbreviation, CRLF, exclamation.

## Fixes applied during conformance run

Only 4 failures in the initial run (out of 512):

1. **U+01BB misclassified as Lower** (2 failures). `is_ll_in_latin_extended` used `u % 2 == 1` heuristic which caught Lo characters. Fixed by excluding known Lo code points (U+01BB, U+01C0-01C3).

2. **SB7 firing through Close characters** (2 failures). "etc.)'\u{308}The" should break before 'T', but SB7 prevented it because it didn't check whether Close characters intervened between ATerm and Upper. Fixed by adding `after_close` flag to `SbCtx::SATerm`; SB6/SB7 only apply when `after_close == false`.

## Files changed

- `src/data/sentence_break.rs` -- new (Sb property)
- `src/data/mod.rs` -- added sentence_break module
- `src/segment/sentence.rs` -- complete rewrite (was stub returning [0])
- `tests/conformance_sentence.rs` -- new (512 conformance tests)
- `tests/segment_tests.rs` -- added 6 sentence unit tests

## Test results

- Grapheme conformance: 766/766 pass
- Word conformance: 1944/1944 pass
- Sentence conformance: 512/512 pass
- Unit tests: 16/16 pass

## Commands

- Full suite: `cargo test`
- Sentence only: `cargo test --test conformance_sentence`

## Phase 1 status

Week 1-2 (Text Segmentation) is now **complete**. All three UAX #29 segmentation algorithms pass official conformance tests.

Next: **Week 2-3 -- Normalization (UAX #15)**: NFC, NFD, NFKC, NFKD, stream-safe, quick check.
