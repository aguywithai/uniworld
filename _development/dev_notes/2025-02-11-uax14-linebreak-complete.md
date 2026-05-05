# Dev notes -- 2025-02-11: UAX #14 Line Breaking -- COMPLETE

## Results

- **LineBreakTest.txt**: 19,338/19,338 passed (Unicode 17.0, UAX #14 revision 55)
- No regressions in any other conformance suite (bidi, grapheme, word, sentence, normalization all green).

## Files created/modified

### New files
- `_development/scripts/generate_linebreak_tables.py` -- generates Rust data tables from
  LineBreak.txt (Line_Break property) and EastAsianWidth.txt (East_Asian_Width).
- `src/data/line_break.rs` (auto-generated) -- Lb enum (43 line break classes), range-based
  lb() lookup (4200+ ranges), is_east_asian_wide() lookup, QU_Pi/QU_Pf helpers.
- `src/linebreak/mod.rs` -- full UAX #14 rule-based implementation.
- `tests/conformance_linebreak.rs` -- conformance test against LineBreakTest.txt.

### Modified files
- `_development/scripts/download_ucd_tests.ps1` -- added LineBreak.txt, LineBreakTest.txt,
  EastAsianWidth.txt downloads.
- `src/data/mod.rs` -- added `pub mod line_break`.
- `src/lib.rs` -- added `pub mod linebreak`.
- `.cursor/rules/PROJECT_ROADMAP.mdc` -- marked UAX #14 complete.

## Algorithm architecture

The implementation follows UAX #14 revision 55 (Unicode 17.0) strictly:

1. **LB1**: Resolve ambiguous classes (AI->AL, CJ->NS, SA->AL/CM based on
   General_Category, SG->AL, XX->AL).
2. **LB2-LB3**: sot x, x eot (always break at end).
3. **LB4-LB6**: Mandatory breaks after BK; CR x LF; break after CR/LF/NL.
4. **LB7**: x SP, x ZW (no break before space or ZWSP).
5. **LB8**: ZW SP* / (break after zero-width space + spaces).
6. **LB8a**: ZWJ x (don't break after ZWJ).
7. **LB9-LB10**: CM/ZWJ absorption -- treat X CM* as X; standalone CM/ZWJ -> AL.
8. **LB11-LB12a**: WJ (word joiner) non-breaking; GL (glue) non-breaking.
9. **LB13**: x CL, x CP, x EX, x SY (close/exclamation/symbol).
10. **LB15.3/LB15.4**: IS (InfixSep) handling -- SP / IS NU exception; otherwise x IS.
11. **LB15a/LB15b**: Context-sensitive QU_Pi/QU_Pf before/after quotation marks.
12. **LB16**: CL/CP SP* x NS.
13. **LB17**: B2 SP* x B2 (break before and after).
14. **LB18**: SP / (spaces end non-breaking contexts).
15. **LB19**: Context-sensitive quotation mark rules (Unicode 17.0):
    - 19.01: x QU-not-Pi, 19.02: QU-not-Pf x
    - 19.1/19.11: [^EastAsian] x QU / x QU ([^EastAsian]|eot)
    - 19.12/19.13: QU x [^EastAsian] / ([^EastAsian]|sot) QU x
16. **LB20a**: Word-initial hyphens: (sot|BK|CR|LF|NL|OP|QU|GL|SP|ZW|CB) (HY|HH) x (AL|HL).
    CM-transparent: uses find_base_index to look past absorbed CMs.
17. **LB21/LB21a/LB21b**: x BA/HY/NS, BB x; HL (HY|HH) x [^HL]; SY x HL.
18. **LB22-LB24**: x IN; (AL|HL) x NU / NU x (AL|HL); PR/PO x ID/EB/EM etc.
19. **LB25**: Complex numeric context (PR/PO x OP/HY? NU; NU (SY|IS)* x NU;
    NU (SY|IS)* (CL|CP)? x PR/PO).
20. **LB26-LB27**: Korean Jamo/Hangul interactions (JL/JV/JT/H2/H3).
21. **LB28/LB28a**: AL x AL; aksara/Brahmic script rules:
    - 28.11: AP x (AK|dotted_circle)
    - 28.12: (AK|AS|dotted_circle) x (VF|VI)
    - 28.13: (AK|AS|dotted_circle) VI x (AK|dotted_circle) -- CM-transparent lookback
    - 28.14: (AK|AS|dotted_circle) x (AK|AS|dotted_circle) VF -- lookahead past CMs
22. **LB29**: IS x (AL|HL).
23. **LB30**: (AL|HL|NU) x OP [non-EAW]; CP [non-EAW] x (AL|HL|NU).
24. **LB30a**: Regional indicator pairing -- (RI RI)* RI x RI.
    CM-transparent: backward RI count only counts positions with original class RI.
25. **LB30b**: EB x EM; ExtPict_Extend* x EM.
26. **LB31**: ALL / ALL (default: allow break).

## Key fixes during development

The path to full conformance required multiple rounds of iterative fixes.

### Phase 1: Initial implementation (~3400 failures -> 213)
- Basic rule skeleton with LB1-LB31.
- First pass at CM/ZWJ absorption.
- Major rule ordering issues resolved.

### Phase 2: Rule refinements (213 -> 106 -> 27)
- LB9/LB10 absorption: ensure standalone CM at sot maps to AL.
- LB25 numeric context: forward scan pattern for (PR|PO) x OP? HY? NU
  and backward pattern for NU (SY|IS)* (CL|CP)? x (PR|PO).
- LB15a/LB15b: QU with Pi/Pf property classification and surrounding context.

### Phase 3: Final 16 -> 0 (this session, frontier model)

1. **LB30a CM transparency** (1 failure, line 17949): Backward RI count was counting
   CM/ZWJ positions that absorbed into RI. Fix: only count positions where
   `classes[j] == Lb::RI` (original class), not effective class.

2. **LB13/LB15 IS reclassification** (2 failures, lines 19123/19343): IS (InfixSep)
   was unconditionally prohibited by LB13. Unicode 17.0 requires context: allow
   `SP / IS NU` (LB15.3) while maintaining general `x IS` (LB15.4).

3. **LB19 context-sensitive QU** (7 failures, lines 19347-19353): Old blanket
   `x QU / QU x` replaced with six sub-rules (19.01/19.02/19.1/19.11/19.12/19.13)
   that distinguish QU_Pi from QU_Pf and apply East Asian width awareness.

4. **LB20a word-initial hyphens + CM transparency** (4 + 18 failures): Two issues.
   (a) The "word-initial" condition was too broad (negation of alpha/nu); narrowed to
   explicit preceding context set (sot|BK|CR|LF|NL|OP|QU|GL|SP|ZW|CB).
   (b) When HH/HY had absorbed CMs, the context check looked at `effective[i-1]`
   which was the absorbed CM position. Fix: use `find_base_index` to look past the
   CM cluster to the real preceding context. This was the largest batch of failures
   (18 regressions introduced then fixed in one step).

5. **LB21a Hebrew hyphen update** (2 failures, lines 19326/19362): Old rule
   `HL (HY|BA) x` updated to Unicode 17.0's `HL (HY|HH) x [^HL]` -- BA replaced
   with HH, and the rule now requires the following character NOT to be HL.

6. **LB28a aksara CM transparency** (1 failure, line 19329): Rule 28.13
   `(AK|AS|dotted_circle) VI x (AK|dotted_circle)` was checking `i-1` for the
   aksara before VI. When the VI had absorbed CMs (e.g. ZWNJ in Balinese), the
   check found VI instead of the aksara. Fix: use `find_base_index` to look past
   the CM cluster to the actual VI base, then check one position further back.

## Complexity notes

UAX #14 is a broad, detail-heavy algorithm. Key sources of complexity:

- **43 character classes** with many context-dependent interactions (vs. 23 for bidi,
  ~15 for word/sentence break).
- **CM/ZWJ transparency** pervades the algorithm: LB9 absorption means every rule
  that checks context (preceding or following characters) must be aware that
  intervening CMs may mask the true surrounding classes. This was the root cause
  of the most persistent bugs (LB20a, LB28a, LB30a).
- **Unicode 17.0 rule revisions**: Several rules changed between Unicode versions --
  LB19 (QU context sensitivity), LB20a (word-initial hyphens with HH), LB21a
  (HH replacing BA), LB15 (IS reclassification). Implementations based on older
  spec text will fail the latest conformance tests.
- **Aksara/Brahmic scripts** (LB28a): A sub-algorithm within the main algorithm,
  requiring virama/joiner-aware lookahead and lookback.
- **Regional indicator pairing** (LB30a): Similar to WB15/16 but with the added
  complexity of CM-transparent counting.

Despite a large rule count, the algorithm is fundamentally a left-to-right scan with
some fixed-distance lookback and lookahead. It does not have the stack-based scoping
or run-sequence construction of bidi (UAX #9), making it structurally simpler but
requiring more attention to edge cases.

## Session cost and development style

This completion required a frontier model (Opus-class) working iteratively over
multiple rounds:
- Total wall-clock time: approximately 1.5-2 hours of model interaction
- Approximate cost: ~$25 in API usage
- The complexity was concentrated in the final 16 failures, where spec rule
  interpretation, CM transparency interactions, and Unicode 17.0 rule changes
  all intersected. The initial ~19,000 passing required careful but straightforward
  implementation; the last 16 required deep understanding of rule interactions.

## Status

Phase 1 is now **100% complete**. All four core Unicode algorithms pass full
conformance against Unicode 17.0 test data:

| Algorithm | Spec | Test cases |
|-----------|------|------------|
| Text Segmentation (Grapheme) | UAX #29 | 766/766 |
| Text Segmentation (Word) | UAX #29 | 1,944/1,944 |
| Text Segmentation (Sentence) | UAX #29 | 512/512 |
| Normalization (NFC/NFD/NFKC/NFKD) | UAX #15 | ~20,000+ |
| Bidirectional Algorithm | UAX #9 | 861,948/861,948 |
| Line Breaking | UAX #14 | 19,338/19,338 |
