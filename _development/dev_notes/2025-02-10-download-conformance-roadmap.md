# Dev notes — 2025-02-10: Download, conformance run, roadmap status

**Current status:** Grapheme conformance is now **766/766** (GB11 and ConjunctLinker done). See `2025-02-10-grapheme-complete-phase1-status.md` for what was accomplished and Phase 1 / roadmap percentage.

## Download and conformance

- **download_ucd_tests.ps1** ran successfully (full permissions; sandbox had closed connection). GraphemeBreakTest.txt is in _development/data/ucd/.
- **Conformance test** ran with real data: **745 passed, 21 failed** (766 cases total).
- **Fixes applied during run:** (1) ZWJ: no break before ZWJ (GB9 × ZWJ). (2) Prepend: added U+06DD and U+0600..U+0605 to GCB; GB9b Prepend × (not Control/CR/LF) = no break; break after Prepend only for Control/CR/LF. (3) Removed incorrect "base × Prepend = no break" so we break between base and Prepend per spec.

## Remaining 21 failures

- **Emoji (GB11):** ZWJ × Extended_Pictographic — e.g. baby + skin tone + ZWJ + baby, emoji ZWJ sequences. Need Extended_Pictographic property and GB11 rule.
- **Devanagari:** ConjunctLinker (U+094D virama, U+093C nukta) + ZWJ forming single cluster (e.g. क् + त). May need ConjunctLinker in GCB and/or tailored rule.

## Roadmap

- **Phase 0:** Complete (all items checked).
- **Phase 1 — UAX #29:** In progress. Grapheme: 745/766 conformance; word/sentence stubbed. Normalization (UAX #15), Bidi (UAX #9), Line break (UAX #14) not started.
- **Document Generation:** README, docs/, CONTRIBUTING done; API reference not yet.
- Roadmap file updated with current grapheme status and remaining 21 failures.

## Commands

- Download: `.\_development\scripts\download_ucd_tests.ps1`
- Test: `cargo test` (segment + conformance). Conformance fails until 21 cases fixed or test relaxed for known gaps.
