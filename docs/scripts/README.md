# Script-specific usage and tests

This directory is for **per-script usage notes and tests** that complement the main API docs and the Unicode showcase file.

The primary “live” showcase document is:

- `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md`

That file contains representative text samples for many writing systems. The notes here describe how to use those samples when testing UniWorld.

## 1. Tier 1 scripts and UniWorld

Tier 1 scripts (for which UniWorld aims for especially robust behavior) include:

- **Latin and extensions** (accents, ligatures, compatibility forms).
- **Greek and Cyrillic** (case mapping, final sigma).
- **Right-to-left scripts**: Arabic, Hebrew (bidi and cursor movement).
- **Indic and Brahmic scripts**: Devanagari, Bengali, Gurmukhi, Tamil, Sinhala, etc. (conjuncts, virama handling).
- **Southeast Asian no-space scripts**: Thai, Lao, Khmer, Myanmar (dictionary-based line breaking).
- **CJK**: Chinese, Japanese, Korean (full-width vs ASCII width, segmentation).
- **Emoji and symbols**: ZWJ sequences, flags, skin tones, box drawing.

## 2. How to test a script with UniWorld

For any script sample from the showcase file:

1. **Segmentation**  
   - Run grapheme, word, and sentence boundary functions on the sample.
   - Verify expected clustering (no broken emoji ZWJ sequences; Indic conjuncts intact; regional indicator pairs as single clusters).

2. **Line breaking**  
   - Apply `line_break_opportunities` / `line_break_opportunities_with_dictionary` (where relevant).
   - Confirm that line breaks avoid splitting inside grapheme clusters and respect dictionary-based segmentation for Thai/Lao/Khmer/Myanmar.

3. **Normalization**  
   - Compare NFC vs NFD vs NFKC vs NFKD on samples with combining marks and compatibility characters (ligatures, fractions).
   - Ensure canonically equivalent strings compare equal after normalization.

4. **Width and truncation**  
   - Use `display_width` and `truncate_display_width` on CJK/emoji-rich strings to check visual truncation in terminal-like contexts.

5. **Cursor and selection**  
   - For mixed BiDi samples, test both logical and visual cursor movement and word selection.

These patterns apply across all Tier 1 scripts, with the showcase document providing the concrete strings to use.

## 3. Future per-script docs

If needed, additional markdown files can be added here, such as:

- `latin.md` — details on normalization and ligatures.
- `rtl.md` — bidi pitfalls and examples.
- `indic.md` — conjunct clusters, virama rules, and cursor behavior.
- `se_asian.md` — dictionary-based line breaking examples and expected breaks.

For Phase 3, the combination of `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md` and this overview is sufficient to guide script-focused testing and documentation.

