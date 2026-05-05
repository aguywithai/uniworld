# Dev notes — 2025-02-10: Grapheme complete; Phase 1 status and roadmap

## What we accomplished

### Grapheme cluster segmentation (UAX #29) — complete

- **Conformance:** 766/766 GraphemeBreakTest.txt cases pass. All official UAX #29 grapheme break tests pass.
- **GB11 (emoji ZWJ):** Added `Extended_Pictographic` GCB and `is_extended_pictographic()`; ZWJ x Extended_Pictographic = no break only in emoji context (prev is ExtPict or Extend). Emoji skin tones (1F3FB..1F3FF) as Extend; Dingbats 2700..27BF excluded so e.g. U+2701 stays Other and "scissors ZWJ scissors" breaks as expected.
- **ConjunctLinker / IndicLetter:** Added `ConjunctLinker` and `IndicLetter` GCB; virama/nukta (Devanagari 094D/093C, Gujarati 0ACD, Myanmar 1039/103A, Balinese 1B01/1B44, Khmer 17D2). Rule: no break before ConjunctLinker (attaches to previous); ConjunctLinker x (IndicLetter | Extend | Zwj | SpacingMark | ConjunctLinker) when cluster has an Indic base.
- **Balinese specifics:** U+1B01 (ULU RICEM): break after virama so it does not pull in the next letter. U+1B44 (ADEG ADEG): keep 1B44 + following letter together (including at cluster start). U+1B04 (BALINESE SIGN BISAH) as SpacingMark (GB9a) so it attaches to previous.
- **ZWJ x IndicLetter** in Indic context (e.g. Devanagari क्‍त) kept as one cluster when prev is ConjunctLinker/IndicLetter/Extend.

### Code touched

- `src/data/grapheme_break.rs`: Gcb::ExtendedPictographic, ConjunctLinker, IndicLetter; is_extended_pictographic, is_indic_letter; ConjunctLinker list; SpacingMark 0x0AFB, 0x1B04; Extend 0x1037.
- `src/segment/grapheme.rs`: break_between() with GB11, ZWJ x IndicLetter, ExtPict x Extend, x ConjunctLinker, ConjunctLinker x … with 1B01 break and 1B44 no-break; iterator passes prev/next codepoint for Balinese.

## Does this complete Phase 1?

**No.** Phase 1 (Core Algorithms in Rust) includes:

1. **Week 1–2 — Text Segmentation (UAX #29):** Only the **grapheme** part is complete. **Word** and **sentence** boundaries are still stubbed (`word_boundaries` and `sentence_boundaries` return `[0]` only). So Week 1–2 is partially done.
2. **Week 2–3 — Normalization (UAX #15):** Not started.
3. **Week 3–5 — Bidi (UAX #9):** Not started.
4. **Week 5–6 — Line Breaking (UAX #14):** Not started.

So Phase 1 is **in progress**: grapheme segmentation is done; word/sentence and the rest of Phase 1 remain.

## Roadmap percentage (rough)

- **Phase 0:** Complete (100%).
- **Phase 1:** Four week-blocks. One block is “Text Segmentation”; within that, grapheme is one of three (grapheme, word, sentence). So grapheme = ~1/3 of one block → **~8% of Phase 1**.
- **Phases 2–4:** Not started.

**Overall:** Phase 0 done + grapheme done ≈ **~25% of roadmap** (by phase count: one phase full, one phase barely started).

## Significance of the project (from UniWorld_PROJECT.md)

UniWorld aims to be the **missing layer** between the Unicode specification and the ecosystem: one library that implements UAX #9, #14, #29, #15 correctly and is easy to integrate. Billions of users are affected by broken bidi, Indic conjuncts, Thai/Lao line breaking, and emoji/combining sequences. We have completed the **first conformance-grade piece**: extended grapheme cluster boundaries for all scripts covered by GraphemeBreakTest.txt, including emoji ZWJ and Indic ConjunctLinker. That underpins cursor movement, selection, truncation, and display for every script in the test suite.

## Commands

- Conformance: `cargo test --test conformance_grapheme` (expect 766 pass).
- All tests: `cargo test`.

## Next steps (roadmap)

- Implement UAX #29 **word** and **sentence** boundaries (and conformance if available) to finish Week 1–2.
- Then normalization (UAX #15), bidi (UAX #9), line breaking (UAX #14) per PROJECT_ROADMAP.mdc.
