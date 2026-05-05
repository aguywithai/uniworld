# Showcase PDF: Rendering Lessons and Structural vs Unicode Issues

**Context**: The UniWorld Unicode Showcase (`docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md`) is rendered to HTML and PDF for demos and stress-testing. Getting the full document to render correctly in headless PDF revealed that many failures were **not** Unicode or library limitations but **structural** (HTML/CSS/print layout). This note records what we learned for future debugging and for distinguishing "pipeline rendering" from "UniWorld correctness."

---

## Pipeline

- **Source**: `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md`
- **HTML**: Python `markdown` library (extensions `extra`) -> wrapped in minimal UTF-8 template -> `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.html`
- **PDF**: Chrome (preferred) or Edge, launched with `--remote-debugging-port`, DevTools Protocol `Page.printToPDF`. Script: `_development/scripts/render_unicode_showcase_to_pdf.py`

The CLI flag `--print-to-pdf=...` was unreliable (especially on Edge); CDP `Page.printToPDF` is used instead. Chrome produces more consistent PDFs than Edge for this document.

---

## Structural vs Unicode: What Actually Broke

When sections failed to appear or were clipped in the PDF, the causes fell into two categories.

### 1. Structural / layout (not Unicode)

These had nothing to do with UniWorld or Unicode handling. They were about how the **browser print engine** laid out certain HTML patterns.

| Section | Symptom | Cause | Fix |
|--------|---------|--------|-----|
| **11** (box drawing) | Shaded/blank block or content after it missing | **Emoji inside `<pre>`**: checkmark emoji (U+2705) inside a monospace box-drawing block. Color emoji have different metrics than monospace glyphs; print layout miscalculated block height and broke page flow. | Replaced checkmarks with ASCII `[x]` in the pre block. Box-drawing Unicode (U+2500 range) was never the problem. |
| **12** (mixed-script paragraph) | Long paragraph not wrapping or clipping | **Entire paragraph was one inline `<code>`** (markdown `` `...` ``). One unbreakable inline element; print could not wrap it. | Removed backtick wrapping so the blockquote contains normal text. All mixed-script Unicode unchanged; only the HTML structure changed. |
| **13** (how to use) | Section missing or clipped | **Downstream of 11/12**: once 11 or 12 broke layout (wrong heights, overflow), the engine clipped or pushed content; 13 appeared broken as a side effect. | Fixed by fixing 11 and 12. |

Takeaway: If "Unicode-heavy" content fails in a **render pipeline** (PDF, editor, etc.), check **structure first**: single long inlines, emoji inside monospace, overflow, page-break rules. The Unicode is often fine.

### 2. Print engine limitations (complex emoji)

These were about **headless print-to-PDF** (Chrome/Edge) having trouble with certain emoji **as rendered**, not with UniWorld's handling of the codepoints.

| Section | Symptom | Cause | Fix |
|--------|---------|--------|-----|
| **9** (emoji) | Only part of section 9 visible; break often after first ZWJ or dense flags | **ZWJ sequences** (e.g. family emoji) and **dense regional-indicator sequences** are multi-codepoint, color glyphs. Print layout miscalculated line/block heights and broke page flow. | Section 9.3: describe ZWJ sequences by **codepoint** (U+1F468 ZWJ U+1F469 ...) instead of embedding the rendered ZWJ emoji. Section 9.4: one flag per line. Added explicit note in doc: omitted for PDF **rendering** reasons, not UniWorld limitations. |

UniWorld correctly segments ZWJ sequences and regional indicator pairs as single grapheme clusters. The limitation is the **print renderer**, not the library. The showcase text now states that.

---

## Changes to the showcase source (summary)

- **Section 9.3**: ZWJ emoji replaced by codepoint descriptions; disclaimer added that rendered ZWJ are omitted due to PDF rendering issues in the demo, not Unicode limitations of UniWorld.
- **Section 9.4**: Flags kept but spaced (one per line) to avoid dense inline sequences that broke layout.
- **Section 11**: Checkmark emoji in box-drawing pre block replaced with `[x]`.
- **Section 12**: Blockquote stress-test paragraph unwrapped from `` `...` `` so it is normal blockquote text (still full mixed script).

No change to the **Unicode coverage** the document demonstrates; only to **markup and to which glyphs we ask the PDF engine to render** in problematic contexts.

---

## For future debugging

1. **HTML first**: Open the generated HTML in the same browser (Chrome/Edge). If it looks correct there, the bug is in **print/PDF path**, not in the document or Unicode.
2. **Simplify structure**: Long single inlines, emoji inside `<pre>`/`<code>`, and dense emoji lines are the main layout traps. Prefer normal paragraphs, one-emoji-per-line where needed, and ASCII in monospace art for print.
3. **Chrome over Edge**: For this script, Chrome's headless PDF is more reliable. The script prefers Chrome when available.
4. **UniWorld vs pipeline**: When something "Unicode" fails in an app or export, separate: (a) does UniWorld segment/handle it correctly? (tests, API) vs (b) does the **rendering pipeline** (font, layout, print) handle the resulting glyphs? This doc's PDF issues were all (b).

---

## Script and requirements

- **Script**: `_development/scripts/render_unicode_showcase_to_pdf.py` (CDP-based; no pandoc, no WeasyPrint).
- **Requirements**: `markdown` only; browser (Chrome or Edge) must be installed.
- **Output**: `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.html`, `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.pdf`.
